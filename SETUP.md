# Plane Worker Daemon Setup Guide

## 1. Requirements
- Rust (for building)
- Caddy (installed and running)
- Systemd
- SQLite

## 2. Build
```bash
cargo build --release
cp target/release/plane /opt/plane/plane
```

## 3. Configuration
1.  Create `/etc/worker/config.yaml` based on `config.yaml`.
2.  Generate a master key:
    ```bash
    openssl rand -hex 32 > /etc/worker/master.key
    chmod 600 /etc/worker/master.key
    chown worker:worker /etc/worker/master.key
    ```
    Or use `master.key.sample` for testing.

## 4. Systemd Setup
1.  Copy `worker.service` to `/etc/systemd/system/`.
2.  Copy `app@.service` to `/etc/systemd/system/`.
3.  Reload and start:
    ```bash
    systemctl daemon-reload
    systemctl enable --now worker
    ```

## 5. Caddy Setup
1.  Ensure Caddy is running.
2.  Configure `caddyfile_path` in `config.yaml` to point to the live Caddyfile (e.g., `/etc/caddy/Caddyfile`).
3.  Ensure the `worker` user has write permissions to that Caddyfile or use `caddy` group.

## 6. Usage API
All requests must be signed with `X-Signature`.

### Add Project
**Endpoint**: `POST /projects`
**Secret**: Worker Secret (global)
**Payload**:
```json
{
  "project_id": "my-app",
  "domains": ["app.example.com"],
  "encrypted_env": "HEX_ENCODED_AES_GCM_ENCRYPTED_DATA",
  "webhook_secret": "random_string"
}
```

### Deploy App
**Endpoint**: `POST /webhook/deploy/my-app`
**Secret**: Webhook Secret (per project)
**Payload**:
```json
{
  "url": "https://s3.bucket/bundle.zip"
}
```

## 7. Generating Signatures (Client Side)
Node.js Example:
```javascript
const crypto = require('crypto');
const secret = 'YOUR_SECRET';
const body = JSON.stringify(payload);
const signature = crypto.createHmac('sha256', secret).update(body).digest('hex');
// Header: X-Signature: <signature>
```
