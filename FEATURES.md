# 🌟 Zexio Agent Features

Zexio Agent is a lightweight, secure, and production-ready worker daemon for deploying and managing applications on a VPS. It provides a REST API to manage the entire lifecycle of your projects.

## 🚀 Core Features

### 1. Project Management
*   **Create Projects**: Initialize new projects with isolated environments.
*   **Delete Projects**: Clean up resources (files, database entries, systemd services) with a single API call.
*   **List Projects**: View all active projects on the worker.

### 2. Flexible Deployment
*   **URL-Based Deploy**: Deploy applications directly from a URL (e.g., S3 presigned URL).
*   **File-Based Deploy (Rollback)**: Redeploy using an existing artifact already present on the server (useful for instant rollbacks).
*   **Webhook Support**: CI/CD integration via webhook endpoints to trigger deployments automatically.
*   **Zero-Downtime Updates**: Automatic service restart via Systemd.

### 3. Configuration & Security
*   **Encrypted Secrets**: Environment variables are encrypted using **AES-GCM** before being stored in the database.
*   **HMAC Authentication**: All API requests are secured with `X-Signature` header using HMAC-SHA256 and a strict Worker Secret.
*   **Automatic TLS**: Integrated with **Caddy** for automatic HTTPS and domain management.

### 4. Observability & Monitoring
*   **Real-time Stats**: Monitor global CPU and RAM usage of the worker server.
*   **Service Status**: Check if a project is `active`, `running`, or `failed`, and view its PID.
*   **Log Streaming**:
    *   **Project Logs**: View the last 100 lines of logs for any specific application.
    *   **System Logs**: View the worker daemon's own system logs for debugging.

### 5. Service Management
*   **Dependency Control**: Install or remove supported services directly via API:
    *   Node.js (LTS)
    *   Redis
    *   PostgreSQL
*   **Version Check**: Verify installed versions of services programmatically.

## 📂 System Architecture
*   **Process Manager**: Uses native `systemd` for robust process management.
*   **Reverse Proxy**: Uses **Caddy** for routing and SSL termination.
*   **Database**: Uses embedded **SQLite** for lightweight state management.
*   **Storage**: Organized directory structure (`/zexio/apps/{id}/bundle`) for easy manual inspection if needed.
