#!/bin/bash
set -e

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
apt-get update -qq > /dev/null
apt-get install -y -qq curl wget sqlite3 unzip ufw jq gnupg2 lsb-release > /dev/null

# --- Optional Services Helper Functions ---

install_nodejs() {
  if ! command -v node &> /dev/null; then
      echo "🟢 Installing Node.js LTS..."
      curl -fsSL https://deb.nodesource.com/setup_lts.x | bash - > /dev/null
      apt-get install -y nodejs > /dev/null
      # Install PM2/Yarn optionally? Let's stick onto core node.
      npm install -g yarn pnpm pm2
      echo "✅ Node.js $(node -v) Installed"
  else
      echo "✅ Node.js already installed"
  fi
}

install_postgres() {
  if ! command -v psql &> /dev/null; then
      echo "🐘 Installing PostgreSQL..."
      apt-get install -y postgresql postgresql-contrib > /dev/null
      systemctl enable --now postgresql
      echo "✅ PostgreSQL Installed"
  else
      echo "✅ PostgreSQL already installed"
  fi
}

install_redis() {
  if ! command -v redis-server &> /dev/null; then
      echo "🔴 Installing Redis..."
      apt-get install -y redis-server > /dev/null
      # Configure to use systemd supervision if needed
      sed -i 's/^supervised no/supervised systemd/' /etc/redis/redis.conf
      systemctl restart redis.service
      systemctl enable --now redis-server
      echo "✅ Redis Installed"
  else
      echo "✅ Redis already installed"
  fi
}

# --- Service Selection ---
# Check if running interactively
if [ -t 0 ]; then
    echo ""
    echo "❓ Would you like to install additional services?"
    
    read -p "   Install Node.js (LTS)? [y/N] " -r
    [[ $REPLY =~ ^[Yy]$ ]] && install_nodejs

    read -p "   Install PostgreSQL? [y/N] " -r
    [[ $REPLY =~ ^[Yy]$ ]] && install_postgres

    read -p "   Install Redis? [y/N] " -r
    [[ $REPLY =~ ^[Yy]$ ]] && install_redis
    echo ""
else
    # Non-interactive mode (use ENV vars)
    [ "$INSTALL_NODE" == "true" ] && install_nodejs
    [ "$INSTALL_POSTGRES" == "true" ] && install_postgres
    [ "$INSTALL_REDIS" == "true" ] && install_redis
fi
# ---------------------------

# 3. Create Users & Directories
echo "📂 Setting up paths..."
id -u worker &>/dev/null || useradd -r -s /bin/false worker

mkdir -p /app/vectis
mkdir -p /apps
mkdir -p /etc/vectis

chown -R worker:worker /app/vectis /apps
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
LATEST_RELEASE=$(curl -s https://api.github.com/repos/YOUR_GITHUB_USERNAME/vectis-node/releases/latest | grep "tag_name" | cut -d '"' -f 4)

if [ -n "$LATEST_RELEASE" ]; then
    curl -L -o /app/vectis/plane "https://github.com/YOUR_GITHUB_USERNAME/vectis-node/releases/download/${LATEST_RELEASE}/vectis-node"
    echo "✅ Downloaded vectis-node ${LATEST_RELEASE}"
else
    echo -e "${RED}⚠️  Failed to fetch latest release. Please download manually.${NC}"
    exit 1
fi

chmod +x /app/vectis/plane

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
WorkingDirectory=/app/vectis
ExecStart=/app/vectis/plane
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

    RESPONSE=$(curl -s -w "\n%{http_code}" -X POST "$DASHBOARD_URL/api/v1/workers" \
      -H "Authorization: Bearer $VECTIS_TOKEN" \
      -H "Content-Type: application/json" \
      -d "{\"name\": \"$HOSTNAME\", \"ip\": \"$PUBLIC_IP\", \"secret\": \"$WORKER_SECRET\", \"port\": 3000}")
    
    HTTP_CODE=$(echo "$RESPONSE" | tail -n1)
    if [[ "$HTTP_CODE" =~ ^2 ]]; then
        echo -e "${GREEN}✅ Registered successfully!${NC}"
    else
        echo -e "${RED}❌ Registration failed (HTTP $HTTP_CODE)${NC}"
    fi
fi

echo -e "${GREEN}✨ Installation Complete! Plane is flying.${NC}"
