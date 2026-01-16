# Changelog

All notable changes to the **Zexio Platform** (Agent, Backend, Infrastructure) will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2026-01-16

### 🚀 New Features
- **Native gRPC Tunnel**: Integrated high-performance gRPC tunnel client (Tonic) directly into the main Agent binary.
- **Hybrid Architecture**: Unified Management API (Axum) and Tunnel Client (gRPC) into a single process.
- **Protocol Parity**: Implemented `node_sync.proto` v1 for seamless synchronization with Zexio Relay.

### 🛠️ Changes
- **Dependencies**: Added `tonic`, `prost`, `pingora`, and `async-stream` to core dependencies.
- **Refactor**: Restored legacy `src_legacy` features (GUI support, REST API) while keeping the new tunnel logic.
- **Naming**: Renamed directory structure to `zexio_agent`.

## [0.1.0] - 2026-01-14

### 🚀 New Features
- **Agent Auto-Registration**: Implemented secure token-based registration flow (`curl ... | bash -s -- --token=...`).
- **Standalone Mode**: Added support for running the agent offline/locally via `--standalone` flag.
- **Multi-Architecture Support**: Updated CI/CD (`target.yml`) to support Linux (AMD64/ARM64), Windows, and macOS.
- **Dynamic Configuration**: Added support for `ZEXIO_CLOUD__API_URL` and `ZEXIO_CLOUD__TOKEN` environment variables to override defaults.
- **Backend**: Added secure `POST /workers/register` endpoint with `ProvisioningGuard`.

### 🛠️ Changes
- **Rebranding**: Renamed core binary from `plane` to `zexio_agent`.
- **Installer**: Refactored `install.sh` to handle new flags and persist identity to `/etc/zexio/`.
- **Storage**: Clarified that the Agent uses **JSON/File System** storage (previously incorrectly documented as SQLite).
- **Docs**: Comprehensive update to `plane/README.md` covering build, install, and config steps.

### 🐛 Bug Fixes
- **Backend Service**: Restored missing `registerUsingToken` method in `WorkersService` that caused compilation errors.
- **Agent Stability**: Fixed crash when identity file was missing (now defaults to Standalone warning).
- **CI/CD**: Fixed broken binary naming in release workflow.

### 🔧 Infrastructure
- **CI/CD**: Implemented Matrix Build Strategy for cross-platform release artifacts.
- **Cleanup**: Removed legacy documentation and roadmap files (`work_todos`, `busines_roadmap`) to declutter the repository.
