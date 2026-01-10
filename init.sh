#!/bin/bash
set -e

echo "✈️  Initializing Plane Worker Setup (Local Dev Mode)..."
echo "⚠️  Note: Production uses /etc/vectis, /app/vectis, /apps"

# 1. Generate Local Keys
if [ ! -f "master.key" ]; then
    echo "🔑 Generating local master.key..."
    openssl rand -hex 32 > "master.key"
    chmod 600 "master.key"
fi

if [ ! -f "worker.secret" ]; then
    echo "🔑 Generating local worker.secret..."
    openssl rand -hex 24 > "worker.secret"
    chmod 600 "worker.secret"
fi

# 2. Setup Local Config
if [ ! -f "config.yaml" ]; then
    echo "📝 Creating local config.yaml overriding defaults..."
    cat > "config.yaml" <<EOL
server:
  port: 3000
  host: "0.0.0.0"

storage:
  # Override absolute defaults with local relative paths for testing
  database_url: "sqlite://./plane.db"
  projects_dir: "./projects"

caddy:
  admin_api: "http://localhost:2019"
  caddyfile_path: "./Caddyfile"

secrets:
  # Load from local files
  master_key_path: "./master.key"
  worker_secret_path: "./worker.secret"
EOL
    echo "✅ Generated config.yaml for local testing"
fi

mkdir -p projects
echo "✅ Created local projects dir"

# 3. Auto-Registration (Optional)
DASHBOARD_URL="https://dashboard.vectis.dev" # Change this if self-hosting the dashboard

if [ -n "$VECTIS_TOKEN" ]; then
    echo "🔗 Auto-registering with Dashboard at $DASHBOARD_URL..."
    
    # Get Public IP (or use generic 'localhost' for fallback)
    PUBLIC_IP=$(curl -s https://api.ipify.org || echo "127.0.0.1")
    WORKER_SECRET=$(cat worker.secret)
    
    # Assuming Dashboard Endpoint: POST /api/v1/workers
    RESPONSE=$(curl -s -w "\n%{http_code}" -X POST "$DASHBOARD_URL/api/v1/workers" \
      -H "Authorization: Bearer $VECTIS_TOKEN" \
      -H "Content-Type: application/json" \
      -d "{\"name\": \"$HOSTNAME\", \"ip\": \"$PUBLIC_IP\", \"secret\": \"$WORKER_SECRET\", \"port\": 3000}")
    
    HTTP_CODE=$(echo "$RESPONSE" | tail -n1)
    BODY=$(echo "$RESPONSE" | sed '$d')

    if [[ "$HTTP_CODE" =~ ^2 ]]; then
        echo "✅ Successfully registered worker!"
    else
        echo "❌ Registration Failed (Status $HTTP_CODE): $BODY"
        # Don't fail the entire script, just warn
    fi
else
    echo "ℹ️  Skipping auto-registration (VECTIS_TOKEN not set)"
fi

echo ""
echo "🎉 Local Init Complete. Run with: cargo run"
