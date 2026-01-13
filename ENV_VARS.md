# Environment Variables for Zexio Agent

## Required Environment Variables

None! The node can start with zero environment variables using all defaults.

## Optional Environment Variables

### Server Configuration

| Variable | Default | Description |
|----------|---------|-------------|
| `SERVER_PORT` | `3000` | HTTP server port |
| `RUN_MODE` | `production` | Runtime mode (production/development) |
| `DOWNLOAD_URL` | None | Custom direct link to download the binary during installation |

### Secrets (Auto-loaded from files)

The node automatically reads secrets from these file paths:

| File Path | Purpose |
|-----------|---------|
| `/etc/vectis/worker.secret` | HMAC authentication secret |
| `/etc/vectis/master.key` | Encryption master key |

**Note:** These files are created automatically by `install.sh`

### Public Access (Optional)

| Variable | Default | Description |
|----------|---------|-------------|
| `PUBLIC_HOSTNAME` | None | Public domain for this worker (e.g., `worker1.example.com`) |
| `PUBLIC_IP` | None | Public IP address |

## Default Configuration

If no environment variables are set, the node uses:

```bash
SERVER_PORT=3000
STORAGE_DIR=/apps
CONFIG_DIR=/etc/vectis
CADDY_ADMIN_API=http://localhost:2019
```

## Minimal Start Command

```bash
# Start with all defaults
./zexio-agent

# Start on custom port
SERVER_PORT=8080 ./zexio-agent
```

## Production Systemd Service

The `install.sh` script creates a systemd service with:

```ini
[Service]
Environment="SERVER_PORT=3000"
ExecStart=/zexio/app/zexio-agent
WorkingDirectory=/zexio/app
```

**No additional environment variables needed!** All secrets are loaded from files.
