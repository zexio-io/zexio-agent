#!/bin/bash
set -e
START_TIME=$(date +%s)

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${GREEN}✈️  Planting the Plane (Vectis Worker Installer)...${NC}"

# 1. Root Check
if [ "$EUID" -ne 0 ]; then 
  echo -e "${RED}Please run as root (sudo bash)${NC}"
  exit 1
fi

# 2. Dependencies
echo "📦 Installing system dependencies..."
# Fix for potential malformed nodesource lists
rm -f /etc/apt/sources.list.d/nodesource.list
apt-get update -qq > /dev/null
apt-get install -y -qq curl wget sqlite3 unzip ufw jq gnupg2 lsb-release ca-certificates > /dev/null

# --- Service Selection ---
# ---------------------------
# ---------------------------

# 3. Create Users & Directories
echo "📂 Setting up paths..."
id -u worker &>/dev/null || useradd -r -s /bin/false worker

mkdir -p /vectis/app
mkdir -p /vectis/apps
mkdir -p /etc/vectis

chown -R worker:worker /vectis/app /vectis/apps
chown -R worker:worker /etc/vectis
chmod 700 /etc/vectis

# 4. Install Caddy (if not present)
if ! command -v caddy &> /dev/null; then
    echo "🌐 Installing Caddy..."
    apt-get install -y debian-keyring debian-archive-keyring apt-transport-https > /dev/null
    curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/gpg.key' | gpg --dearmor -o /usr/share/keyrings/caddy-stable-archive-keyring.gpg
    curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/debian.deb.txt' | tee /etc/apt/sources.list.d/caddy-stable.list > /dev/null
    apt-get update -qq > /dev/null
    apt-get install -y caddy > /dev/null
    echo "✅ Caddy Installed"
else
    echo "✅ Caddy already installed"
fi

# 5. Download Binary
echo "⬇️  Downloading vectis-node binary..."
if [ -n "$DOWNLOAD_URL" ]; then
    echo "Using custom download URL: $DOWNLOAD_URL"
    if [[ "$DOWNLOAD_URL" == *".tar.gz"* ]]; then
        echo "📦 Detected tarball, downloading and extracting..."
        curl -L -o /tmp/vectis-node.tar.gz "$DOWNLOAD_URL"
        tar -xzf /tmp/vectis-node.tar.gz -C /tmp/
        # Move and rename to /vectis/app/plane
        mv /tmp/vectis-node /vectis/app/plane
        rm /tmp/vectis-node.tar.gz
    else
        curl -L -o /vectis/app/plane "$DOWNLOAD_URL"
    fi
    echo "✅ Downloaded vectis-node from custom URL"
else
    LATEST_RELEASE=$(curl -s https://api.github.com/repos/YOUR_GITHUB_USERNAME/vectis-node/releases/latest | grep "tag_name" | cut -d '"' -f 4)
    if [ -n "$LATEST_RELEASE" ]; then
        curl -L -o /vectis/app/plane "https://github.com/YOUR_GITHUB_USERNAME/vectis-node/releases/download/${LATEST_RELEASE}/vectis-node"
        echo "✅ Downloaded vectis-node ${LATEST_RELEASE}"
    else
        echo -e "${RED}⚠️  Failed to fetch latest release. Please provide DOWNLOAD_URL or download manually.${NC}"
        exit 1
    fi
fi

chmod +x /vectis/app/plane

# 6. Generate Keys (if missing)
echo "🔑 Allocating secrets..."
if [ ! -f "/etc/vectis/master.key" ]; then
    openssl rand -hex 32 > /etc/vectis/master.key
    chmod 600 /etc/vectis/master.key
    chown worker:worker /etc/vectis/master.key
fi

if [ ! -f "/etc/vectis/worker.secret" ]; then
    openssl rand -hex 24 > /etc/vectis/worker.secret
    chmod 600 /etc/vectis/worker.secret
    chown worker:worker /etc/vectis/worker.secret
fi

# 7. Systemd
echo "⚙️  Configuring Systemd..."

cat > /etc/systemd/system/worker.service <<EOF
[Unit]
Description=Plane Worker Daemon
After=network.target

[Service]
Type=simple
User=worker
Group=worker
WorkingDirectory=/vectis/app
ExecStart=/vectis/app/plane
Restart=always
# Secrets loaded from files.
# Port can be overridden:
# Environment=PLANE__SERVER__PORT=3000

[Install]
WantedBy=multi-user.target
EOF

cat > /etc/systemd/system/app@.service <<EOF
[Unit]
Description=App %i
After=network.target

[Service]
Type=simple
User=worker
Group=worker
WorkingDirectory=/apps/%i/bundle
ExecStart=/apps/%i/bundle/app
Restart=always
EnvironmentFile=/apps/%i/bundle/.env

[Install]
WantedBy=multi-user.target
EOF

systemctl daemon-reload
systemctl enable --now worker > /dev/null 2>&1

# 8. Auto-Registration
if [ -n "$VECTIS_TOKEN" ]; then
    echo "🔗 Registering with Vectis Dashboard..."
    DASHBOARD_URL="https://dashboard.vectis.dev"
    PUBLIC_IP=$(curl -s https://api.ipify.org)
    WORKER_SECRET=$(cat /etc/vectis/worker.secret)

    # Simplified payload: only IP and secret
    RESPONSE=$(curl -s -w "\n%{http_code}" -X POST "$DASHBOARD_URL/api/v1/workers" \
      -H "Authorization: Bearer $VECTIS_TOKEN" \
      -H "Content-Type: application/json" \
      -d "{\"ip\": \"$PUBLIC_IP\", \"secret\": \"$WORKER_SECRET\"}")
    
    HTTP_CODE=$(echo "$RESPONSE" | tail -n1)
    BODY=$(echo "$RESPONSE" | head -n -1)
    
    if [[ "$HTTP_CODE" =~ ^2 ]]; then
        # Extract subdomain from response
        SUBDOMAIN=$(echo "$BODY" | jq -r '.subdomain')
        WORKER_ID=$(echo "$BODY" | jq -r '.id')
        
        # Save worker configuration
        cat > /etc/vectis/worker.conf <<EOF
WORKER_ID=$WORKER_ID
SUBDOMAIN=$SUBDOMAIN
PUBLIC_IP=$PUBLIC_IP
SYNC_STATUS=synced
LAST_SYNC=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
EOF
        
        echo -e "${GREEN}✅ Registered successfully!${NC}"
        echo "   Subdomain: $SUBDOMAIN"
        echo "   Worker ID: $WORKER_ID"
    else
        # Registration failed - mark as unsynced
        cat > /etc/vectis/worker.conf <<EOF
WORKER_ID=
SUBDOMAIN=
PUBLIC_IP=$PUBLIC_IP
SYNC_STATUS=failed
LAST_SYNC=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
ERROR_MESSAGE=Registration failed (HTTP $HTTP_CODE)
EOF
        
        echo -e "${RED}❌ Registration failed (HTTP $HTTP_CODE)${NC}"
        echo "   Worker marked as unsynced. Run 'vectis-sync' to retry."
    fi
else
    # No token provided - mark as pending
    cat > /etc/vectis/worker.conf <<EOF
WORKER_ID=
SUBDOMAIN=
PUBLIC_IP=
SYNC_STATUS=pending
LAST_SYNC=
ERROR_MESSAGE=No VECTIS_TOKEN provided during installation
EOF
    
    echo "⚠️  Skipping auto-registration (no VECTIS_TOKEN)"
    echo "   Run 'VECTIS_TOKEN=your_token vectis-sync' to register manually"
fi

# 9. Create sync helper script
cat > /usr/local/bin/vectis-sync <<'SYNCEOF'
#!/bin/bash
set -e

source /etc/vectis/worker.conf 2>/dev/null || true

if [ -z "$VECTIS_TOKEN" ]; then
    echo "Error: VECTIS_TOKEN environment variable required"
    exit 1
fi

DASHBOARD_URL="https://dashboard.vectis.dev"
PUBLIC_IP=$(curl -s https://api.ipify.org)
WORKER_SECRET=$(cat /etc/vectis/worker.secret)

echo "Syncing with dashboard..."

RESPONSE=$(curl -s -w "\n%{http_code}" -X POST "$DASHBOARD_URL/api/v1/workers" \
  -H "Authorization: Bearer $VECTIS_TOKEN" \
  -H "Content-Type: application/json" \
  -d "{\"ip\": \"$PUBLIC_IP\", \"secret\": \"$WORKER_SECRET\"}")

HTTP_CODE=$(echo "$RESPONSE" | tail -n1)
BODY=$(echo "$RESPONSE" | head -n -1)

if [[ "$HTTP_CODE" =~ ^2 ]]; then
    SUBDOMAIN=$(echo "$BODY" | jq -r '.subdomain')
    WORKER_ID=$(echo "$BODY" | jq -r '.id')
    
    cat > /etc/vectis/worker.conf <<EOF
WORKER_ID=$WORKER_ID
SUBDOMAIN=$SUBDOMAIN
PUBLIC_IP=$PUBLIC_IP
SYNC_STATUS=synced
LAST_SYNC=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
EOF
    
    echo "✅ Sync successful!"
    echo "   Subdomain: $SUBDOMAIN"
    echo "   Worker ID: $WORKER_ID"
else
    cat > /etc/vectis/worker.conf <<EOF
WORKER_ID=${WORKER_ID:-}
SUBDOMAIN=${SUBDOMAIN:-}
PUBLIC_IP=$PUBLIC_IP
SYNC_STATUS=failed
LAST_SYNC=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
ERROR_MESSAGE=Sync failed (HTTP $HTTP_CODE)
EOF
    
    echo "❌ Sync failed (HTTP $HTTP_CODE)"
    exit 1
fi
SYNCEOF

chmod +x /usr/local/bin/vectis-sync

# 10. Create status check script
cat > /usr/local/bin/vectis-status <<'STATUSEOF'
#!/bin/bash

if [ ! -f /etc/vectis/worker.conf ]; then
    echo "Status: Not configured"
    exit 1
fi

source /etc/vectis/worker.conf

echo "Worker Status"
echo "============="
echo "Worker ID:    ${WORKER_ID:-Not assigned}"
echo "Subdomain:    ${SUBDOMAIN:-Not assigned}"
echo "Public IP:    ${PUBLIC_IP:-Unknown}"
echo "Sync Status:  $SYNC_STATUS"
echo "Last Sync:    ${LAST_SYNC:-Never}"

if [ -n "$ERROR_MESSAGE" ]; then
    echo "Error:        $ERROR_MESSAGE"
fi

# Check if worker service is running
if systemctl is-active --quiet worker; then
    echo "Service:      Running"
else
    echo "Service:      Stopped"
fi
STATUSEOF

chmod +x /usr/local/bin/vectis-status

echo -e "${GREEN}✨ Installation Complete! Plane is flying.${NC}"

END_TIME=$(date +%s)
DURATION=$((END_TIME - START_TIME))
echo -e "${GREEN}⏱️  Time taken for server to go live: ${DURATION} seconds${NC}"
echo ""
echo "Commands:"
echo "  vectis-status  - Check worker status"
echo "  vectis-sync    - Sync with dashboard"
