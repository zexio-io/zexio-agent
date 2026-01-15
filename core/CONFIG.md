# Zexio Agent Configuration & Data Locations

The Zexio Agent stores configuration files (secrets, keys) and data (apps, logs) in different locations depending on the Operating System and the execution mode (User vs System/Root).

## Directory Structure

| OS | Type | Mode | Path |
|----|------|------|------|
| **Windows** | Config & Data | All | `C:\ProgramData\Zexio` |
| **macOS** | Config | User | `~/Library/Application Support/Zexio` |
| | Data | User | `~/Library/Application Support/Zexio/data` |
| | Config | System (Root) | `/Library/Application Support/Zexio` |
| | Data | System (Root) | `/Library/Application Support/Zexio/data` |
| **Linux / RPi** | Config | User | `~/.config/zexio` |
| | Data | User | `~/.local/share/zexio` |
| | Config | System (Root) | `/etc/zexio` |
| | Data | System (Root) | `/var/lib/zexio` |

## Key Files

These files are critical for the agent's operation and identity.

| File | Location | Description |
|------|----------|-------------|
| `identity.json` | Config Dir | Stores the agent's unique Node ID and metadata. |
| `master.key` | Config Dir | Encryption key for securing local secrets. |
| `worker.secret` | Config Dir | Token/Secret used to authenticate with Zexio Cloud. |
| `provisioning_token` | Config Dir | Temporary token used during the initial setup. |
| `apps/` | Data Dir | Directory where deployed applications and services are stored. |

## Environment Variables

You can override defaults using environment variables:

- `SERVER_PORT`: Port for the HTTP API (Default: `8081`).
- `MESH_PORT`: Port for the internal Mesh network (Default: `8082`).
- `ZEXIO_CLOUD__API_URL`: Override the Zexio Cloud API URL (Default: `https://api.zexio.io`).
- `RUN_MODE`: `production` or `development`.
