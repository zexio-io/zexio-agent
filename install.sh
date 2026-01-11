#!/bin/bash
set -e
START_TIME=$(date +%s)

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${GREEN}✈️  Planting the Zexio Plane (Worker Installer)...${NC}"

# ... (Previous code for Root Check and Dependencies omitted for brevity) ...

# 1. Root Check
if [ "$EUID" -ne 0 ]; then 
  echo -e "${RED}Please run as root (sudo bash)${NC}"
  exit 1
fi

# 2. Dependencies
echo "📦 Installing system dependencies..."
apt-get update -qq > /dev/null
apt-get install -y -qq curl wget sqlite3 unzip ufw jq gnupg2 lsb-release ca-certificates > /dev/null

# 3. Create Users & Directories
echo "📂 Setting up paths..."
id -u worker &>/dev/null || useradd -r -s /bin/false worker

mkdir -p /zexio/app
mkdir -p /zexio/apps
mkdir -p /etc/zexio

chown -R worker:worker /zexio/app /zexio/apps
chown -R worker:worker /etc/zexio
chmod 700 /etc/zexio

# 4. Install Cloudflared (if CF_TUNNEL_TOKEN is provided)
if [ -n "$CF_TUNNEL_TOKEN" ]; then
    echo "☁️  CF_TUNNEL_TOKEN detected. Installing cloudflared..."
    if ! command -v cloudflared &> /dev/null; then
        curl -L --output cloudflared.deb https://github.com/cloudflare/cloudflared/releases/latest/download/cloudflared-linux-amd64.deb
        dpkg -i cloudflared.deb
        rm cloudflared.deb
        echo "✅ cloudflared installed"
    else
        echo "✅ cloudflared already installed"
    fi

    echo "⚙️  Configuring cloudflared service..."
    cloudflared service install "$CF_TUNNEL_TOKEN"
    systemctl enable --now cloudflared
    echo "✅ cloudflared service started"
fi

# 5. Download Binary
echo "⬇️  Downloading zexio-node binary..."
if [ -n "$DOWNLOAD_URL" ]; then
    echo "Using custom download URL: $DOWNLOAD_URL"
    curl -L -o /zexio/app/plane "$DOWNLOAD_URL"
    echo "✅ Downloaded zexio-node from custom URL"
else
    # Fallback to a placeholder or latest GitHub release (adjusting for Zexio repo later)
    echo "⚠️  DOWNLOAD_URL not provided. Using development placeholder..."
    touch /zexio/app/plane
fi

chmod +x /zexio/app/plane

# 6. Generate Keys (if missing)
echo "🔑 Allocating secrets..."
if [ ! -f "/etc/zexio/master.key" ]; then
    openssl rand -hex 32 > /etc/zexio/master.key
    chmod 600 /etc/zexio/master.key
    chown worker:worker /etc/zexio/master.key
fi

if [ ! -f "/etc/zexio/worker.secret" ]; then
    openssl rand -hex 24 > /etc/zexio/worker.secret
    chmod 600 /etc/zexio/worker.secret
    chown worker:worker /etc/zexio/worker.secret
fi

# 7. Systemd
echo "⚙️  Configuring Systemd..."

cat > /etc/systemd/system/worker.service <<EOF
[Unit]
Description=Zexio Plane Worker
After=network.target

[Service]
Type=simple
User=worker
Group=worker
WorkingDirectory=/zexio/app
ExecStart=/zexio/app/plane
Restart=always

[Install]
WantedBy=multi-user.target
EOF

cat > /etc/systemd/system/app@.service <<EOF
[Unit]
Description=Zexio App %i
After=network.target

[Service]
Type=simple
User=worker
Group=worker
WorkingDirectory=/zexio/apps/%i/bundle
ExecStart=/zexio/apps/%i/bundle/app
Restart=always
EnvironmentFile=/zexio/apps/%i/bundle/.env

[Install]
WantedBy=multi-user.target
EOF

systemctl daemon-reload
systemctl enable --now worker > /dev/null 2>&1

# 8. Auto-Registration (Enterprise/Pro)
if [ -n "$ZEXIO_TOKEN" ]; then
    echo "🔗 Registering with Zexio Dashboard..."
    DASHBOARD_URL="https://dashboard.zexio.app"
    PUBLIC_IP=$(curl -s https://api.ipify.org)
    WORKER_SECRET=$(cat /etc/zexio/worker.secret)

    # Note: Registration logic will be handled by the Rust agent on first boot
    # if ZEXIO_TOKEN is present in environment or config.
    # For now, we'll just log it.
    echo "ZEXIO_TOKEN detected. Agent will attempt auto-registration on startup."
fi

echo -e "${GREEN}✨ Installation Complete! Zexio Plane is flying.${NC}"

END_TIME=$(date +%s)
DURATION=$((END_TIME - START_TIME))
echo -e "${GREEN}⏱️  Time taken for server to go live: ${DURATION} seconds${NC}"
echo ""
echo "Commands:"
echo "  systemctl status worker  - Check agent status"
echo "  journalctl -u worker     - View agent logs"
