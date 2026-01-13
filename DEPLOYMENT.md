# 🚀 Production Deployment Guide

This guide covers the deployment of the Zexio Agent on Linux servers.

## Prerequisites
- Ubuntu 20.04+ or Debian 11+
- Root access
- **OS**: Linux
- **Dependencies**: `caddy`, `systemd`, `sqlite3`

## 📂 Directory Structure
| Path | Purpose |
|---|---|
| `/etc/vectis/` | Configuration & Secrets |
| `/app/vectis/` | Worker Binaries & DB |
| `/apps/` | Project Bundles (e.g., `/apps/{id}/bundle/`) |

## 🛠️ Configuration Defaults

| Setting | Default Value | Notes |
|---|---|---|
| **Database** | `sqlite:///app/vectis/db/plane.db` | Main persistence. |
| **Projects Dir** | `/apps` | Projects will live in `/apps/{id}/bundle`. |
| **Caddyfile** | `/etc/caddy/Caddyfile` | Main Caddy config. |
| **Master Key** | `/etc/vectis/master.key` | Encryption key (File). |
| **Worker Secret** | `/etc/vectis/worker.secret` | API Auth Secret (File). |

## Service Management

The agent runs as a systemd service named `zexio-agent`.

```bash
# Check status
systemctl status zexio-agent

# Restart
systemctl restart zexio-agent

# Logs
journalctl -u zexio-agent -f
```

## 📦 Deployment Steps

### 1. Create Structure & User
```bash
# Create user
sudo useradd -r -s /bin/false worker

# Create App Directories
sudo mkdir -p /app/vectis/db
sudo mkdir -p /apps
sudo chmod 755 /app/vectis
sudo chown -R worker:worker /app/vectis /apps

# Create Config Directory
sudo mkdir -p /etc/vectis
sudo chown -R worker:worker /etc/vectis
sudo chmod 700 /etc/vectis
```

### 2. Secrets
```bash
# Master Key
openssl rand -hex 32 | sudo tee /etc/vectis/master.key
sudo chmod 600 /etc/vectis/master.key
sudo chown worker:worker /etc/vectis/master.key

# Worker Secret (New!)
openssl rand -hex 24 | sudo tee /etc/vectis/worker.secret
sudo chmod 600 /etc/vectis/worker.secret
sudo chown worker:worker /etc/vectis/worker.secret
```

### 3. Install Binary
```bash
scp target/release/plane user@vps:/tmp/
sudo mv /tmp/plane /app/vectis/
sudo chmod +x /app/vectis/plane
```

### 4. Systemd Service
`/etc/systemd/system/worker.service`:
```ini
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
# Secrets are now read from files by default!
# Environment=PLANE__SERVER__PORT=3000

[Install]
WantedBy=multi-user.target
```

### 5. App Service Template
`/etc/systemd/system/app@.service`:
```ini
[Unit]
Description=App %i
After=network.target

[Service]
Type=simple
User=worker
Group=worker
# Note: Projects are in /apps/%i/bundle
WorkingDirectory=/apps/%i/bundle
ExecStart=/apps/%i/bundle/app
Restart=always
EnvironmentFile=/apps/%i/bundle/.env

[Install]
WantedBy=multi-user.target
```
