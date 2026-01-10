# 📡 Plane Worker API Reference

## 🔐 Authentication
Required Header: `X-Signature` (HMAC-SHA256 of body using Worker Secret).

## 🛠️ System & Infrastructure
*   `GET /health`: Health Check. Returns `200 OK`.
*   `GET /stats`: Global CPU/RAM usage. **(SSE Stream)**
    *   Content-Type: `text/event-stream`
    *   Data: JSON `{"cpu_usage": 12.5, "memory_used": ..., "uptime": ...}`
    *   Updates every 1 second.
*   `GET /system/logs`: Worker daemon system logs. **(SSE Stream)**
    *   Content-Type: `text/event-stream`
    *   Data: Log line string.

### Service Management
*   **GET** `/services`: List installed services and versions.
*   **POST** `/services/install`: Install dependencies.
*   **POST** `/services/remove`: Uninstall dependencies.

## 📦 Project Resources
Base Path: `/projects`

### Core Operations
*   **GET** `/projects`: List all projects.
*   **POST** `/projects`: Create a new project.
*   **DELETE** `/projects/:id`: Delete a project.

### Project Details (`/projects/:id/...`)
1.  **Environment**: `POST /projects/:id/env`
2.  **Domain**: `POST /projects/:id/domain`
3.  **Files**: `GET /projects/:id/files`
4.  **Stats**: `GET /projects/:id/stats` (JSON Snaphot)
5.  **Logs**: `GET /projects/:id/logs` **(SSE Stream)**
    *   Content-Type: `text/event-stream`
    *   Data: Log line string.
    *   Streams `journalctl -f` output.

### Deployment
*   **POST** `/projects/:id/deploy`
*   **POST** `/projects/:id/webhook`
