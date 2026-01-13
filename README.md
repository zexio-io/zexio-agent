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
*   **SQLite3**: (Optional CLI) Useful for debugging the local database, though Zexio Agent bundles the customized driver.
*   **Systemd**: Required for managing the lifecycle of the worker and deployed applications.

### 2. Rust Crates (Built-in)
The following libraries are compiled into the binary. You don't need to install them separately, but here is what they do:

#### Core & Web
*   **`tokio`**: The asynchronous runtime that powers the high-concurrency capability of the worker.
*   **`axum`**: A robust, ergonomic web framework (by the Tokio team) used for the HTTP API (`/deploy`, `/projects`).
*   **`tower-http`**: Middleware stack for `axum` (logging, tracing, cors).

#### Database
*   **`sqlx`** (with `sqlite`): Safe, async SQL driver. It handles migrations and interactions with the local `zexio-agent.db` to store project metadata and encrypted environment variables.

#### Security & Cryptography
*   **`aes-gcm`**: Implements Authenticated Encryption (AEAD) to securely store environment variables at rest using the `master.key`.
*   **`hmac` & `sha2`**: Used to verify the `X-Signature` header on all incoming requests, ensuring only the Dashboard can command the worker.
*   **`rand` & `hex`**: Utilities for safe random generation and encoding.

#### Utilities
*   **`reqwest`**: HTTP Client used to download deployment bundles (zip files) from S3/Storage URLs.
*   **`config`**: Handles loading configuration from files (`config.yaml`) and environment variables (`PLANE__...`).
*   **`tracing`**: Structured logging system for easier debugging and observability.
*   **`trust-dns-resolver`**: (Planned/Partial) For verifying CNAME records before accepting new domains.

## Installation

```bash
curl -sL https://get.zexio.com/agent | sudo bash -s -- --token=YOUR_ORG_TOKEN
```

### Manual Run
```bash
./zexio-agent
```
