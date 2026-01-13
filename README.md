# Zexio Agent

Zexio Agent is the workload orchestrator and P2P mesh router for the Zexio infrastructure. It runs on worker nodes to manage applications and facilitate secure p2p networking.

## Features
- **Project Management**: Systemd-based application deployment and lifecycle management.
- **Zexio Mesh**: Integrated P2P networking for secure internal communication.
- **Resource Monitoring**: Real-time telemetry (CPU, Memory, Network).
- **Auto-Update**: Self-updating binary mechanism.

## 📚 Libraries & Dependencies

### 1. System Requirements (Runtime)
These must be installed on the Linux VPS where Zexio Agent runs.
*   **[Caddy](https://caddyserver.com/)**: Automatically manages SSL certificates (Let's Encrypt) and reverse proxies traffic to your apps.
*   **[OpenSSL](https://www.openssl.org/)**: Used by the administration scripts (`init.sh`, `install.sh`) to generate secure random keys (`master.key`, `worker.secret`).

### 2. Rust Crates (Built-in)
The following libraries are compiled into the binary. You don't need to install them separately, but here is what they do:

#### Core & Web
*   **`tokio`**: The asynchronous runtime that powers the high-concurrency capability of the worker.
*   **`axum`**: A robust, ergonomic web framework (by the Tokio team) used for the HTTP API (`/deploy`, `/projects`).
*   **`tower-http`**: Middleware stack for `axum` (logging, tracing, cors).

#### Storage
*   **`serde_json`**: Handles serialization of project metadata and configuration to local JSON files (`config.json`).
*   **`tokio::fs`**: Asynchronous file system operations for safe, non-blocking I/O.

#### Security & Cryptography
*   **`aes-gcm`**: Implements Authenticated Encryption (AEAD) to securely store environment variables at rest using the `master.key`.
*   **`hmac` & `sha2`**: Used to verify the `X-Signature` header on all incoming requests, ensuring only the Dashboard can command the worker.
*   **`rand` & `hex`**: Utilities for safe random generation and encoding.

#### Utilities
*   **`reqwest`**: HTTP Client used to download deployment bundles (zip files) from S3/Storage URLs.
*   **`config`**: Handles loading configuration from files (`config.yaml`) and environment variables (`PLANE__...`).
*   **`tracing`**: Structured logging system for easier debugging and observability.
*   **`trust-dns-resolver`**: (Planned/Partial) For verifying CNAME records before accepting new domains.

## 🛠️ Building & Installation

### Prerequisites
- Rust (latest stable)
- OpenSSL (libssl-dev)
- Build Essentials (gcc, make)

### Build from Source
```bash
cargo build --release
# Binary will be at: target/release/zexio-agent
```

### 🚀 Deployment Options

#### Option A: Auto-Registration (Connected Mode)
Registers the agent with Zexio Cloud for remote management.
```bash
curl -sL https://get.zexio.com/agent | sudo bash -s -- --token=zxp_YOUR_TOKEN
```
- Requires a Provisioning Token (`zxp_...`) from the Dashboard.
- Automatically sets up identity and secure tunnel.

#### Option B: Standalone Mode (Offline/Local)
Runs the agent in isolation without connecting to Zexio Cloud.
```bash
curl -sL https://get.zexio.com/agent | sudo bash -s -- --standalone
```
- No token required.
- Dashboard features (Stats, Remote Deploy) will be disabled.
- Useful for local development or air-gapped environments.

## ⚙️ Configuration

The agent is configured via `config.yaml` or Environment Variables.

| Variable | Description | Default |
|----------|-------------|---------|
| `SERVER_PORT` | Port for Management API | `3000` |
| `MESH_PORT` | Port for Service Proxy | `8080` |
| `ZEXIO_CLOUD__API_URL` | Zexio Cloud API Endpoint | `https://api.zexio.io` |
| `ZEXIO_CLOUD__TOKEN` | Provisioning Token | `None` |

### Manual Installation (Custom)
You can manually run the agent without the installer script by providing the token via environment variable:

```bash
export ZEXIO_CLOUD__TOKEN="zxp_YOUR_TOKEN"
export ZEXIO_CLOUD__API_URL="http://localhost:4000" # Optional: Dev override

./zexio-agent
```
