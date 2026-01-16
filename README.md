# Zexio Agent

Zexio Agent is the workload orchestrator and secure tunnel client for the Zexio infrastructure. It runs on worker nodes to manage applications and facilitate secure tunneling.

## Features
- **Instant Tunneling**: Expose local ports to the internet with `zexio up <port>`
- **Project Management**: Systemd-based application deployment and lifecycle management
- **Zexio Mesh**: Integrated P2P networking for secure internal communication
- **Resource Monitoring**: Real-time telemetry (CPU, Memory, Network)
- **Auto-Update**: Self-updating binary mechanism

## 🚀 Quick Start

### Installation

```bash
# macOS / Linux
curl -sL https://get.zexio.com/agent | bash
```

### Basic Usage

```bash
# Start a tunnel to expose local port 3000
zexio up 3000

# Run management API only (no tunnel)
zexio

# Unregister from Zexio Cloud
zexio unregister

# Show help
zexio --help
```

## 📚 CLI Commands

### `zexio up <PORT>`
Start a secure tunnel to expose a local port to the internet.

**Example:**
```bash
# Expose a local web server on port 3000
zexio up 3000

# Expose a database on port 5432
zexio up 5432
```

**What happens:**
1. Agent connects to Zexio Relay (gRPC)
2. Authenticates using your identity
3. Opens a bidirectional tunnel
4. Forwards all incoming traffic to `127.0.0.1:<PORT>`

### `zexio unregister`
Disconnect this agent from Zexio Cloud and delete local identity.

**Example:**
```bash
zexio unregister
```

**Use cases:**
- Switching to a different organization
- Resetting agent configuration
- Troubleshooting authentication issues

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
*   **`clap`**: Modern CLI argument parser with derive macros.

#### Tunneling & Networking
*   **`tonic`**: High-performance gRPC framework for Relay communication.
*   **`prost`**: Protocol Buffers implementation for efficient serialization.

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
# Binary will be at: target/release/zexio
```

## ⚙️ Configuration

The agent is configured via `config.yaml` or Environment Variables.

| Variable | Description | Default |
|----------|-------------|---------|
| `SERVER_PORT` | Port for Management API | `8081` |
| `MESH_PORT` | Port for Service Proxy | `8082` |
| `RELAY_URL` | Zexio Relay Endpoint | `http://127.0.0.1:50051` |
| `ZEXIO_CLOUD__API_URL` | Zexio Cloud API Endpoint | `https://api.zexio.io` |
| `ZEXIO_CLOUD__TOKEN` | Provisioning Token | `None` |

### Manual Configuration
You can manually run the agent without the installer script by providing the token via environment variable:

```bash
export ZEXIO_CLOUD__TOKEN="zxp_YOUR_TOKEN"
export RELAY_URL="https://relay.zexio.io:50051"

zexio up 3000
```

## 🔒 Security

- **End-to-End Encryption**: All tunnel traffic is encrypted via TLS (gRPC over HTTPS)
- **Token Authentication**: Relay verifies `auth_token` before accepting connections
- **Identity Isolation**: Each agent has a unique `worker_id` and `secret_key`
- **Auto-Reconnect**: Agent automatically reconnects if Relay restarts or network drops

## 📖 Architecture

```
┌─────────────┐         gRPC/TLS          ┌──────────────┐
│ Zexio Agent │ ◄──────────────────────► │ Zexio Relay  │
│ (Your VPS)  │    Bidirectional Stream   │ (Cloud Edge) │
└─────────────┘                           └──────────────┘
       │                                          │
       │ Forwards to                              │ Receives from
       │ 127.0.0.1:<PORT>                         │ Internet
       ▼                                          ▼
┌─────────────┐                           ┌──────────────┐
│ Your App    │                           │ Public URL   │
│ (e.g. :3000)│                           │ (HTTPS)      │
└─────────────┘                           └──────────────┘
```

## 🤝 Contributing

Contributions are welcome! Please open an issue or PR on GitHub.

## 📄 License

MIT License - see LICENSE file for details.
