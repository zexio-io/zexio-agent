# Plane Service Routes Analysis

## 1. Management API (HTTP & SSE)

### Public Routes (No Authentication Required)
These routes are open significantly to facilitate standalone mode or GUI checks.

*   `GET /health`
    *   **Description**: Health check endpoint.
    *   **Response**: `text/plain` "OK"

*   `GET /stats`
    *   **Description**: Snapshot of global system stats.
    *   **Response**: `application/json`
        ```json
        {
            "cpu_usage": 12.5,
            "memory_used": 104857600,
            "memory_total": 17179869184,
            "memory_percent": 0.6,
            "disk_used": 50000000000,
            "disk_total": 100000000000,
            "disk_percent": 50.0
        }
        ```

*   `GET /stats/stream` **(SSE)**
    *   **Description**: Real-time stream of global system stats (2s interval).
    *   **Event Data**: Same JSON structure as `/stats` above.

*   `POST /tunnel/start`
    *   **Description**: Start a secure tunnel (Cloudflare/Pangolin).
    *   **Request**: `application/json`
        ```json
        {
            "provider": "cloudflare",
            "token": "ey...",
            "local_port": 8082 
        }
        ```
    *   **Response**: `application/json`
        ```json
        {
            "status": "success",
            "message": "Started cloudflare tunnel forwarding to port 8082"
        }
        ```

*   `POST /tunnel/stop`
    *   **Description**: Stop the active secure tunnel.
    *   **Response**: `application/json`
        ```json
        {
            "status": "success",
            "message": "Stopped cloudflare tunnel"
        }
        ```

*   `GET /system/logs`
    *   **Description**: Snapshot of recent system logs.
    *   **Response**: `application/json` `[{"timestamp": "...", "level": "INFO", "message": "..."}]`

*   `GET /system/logs/stream` **(SSE)**
    *   **Description**: Real-time stream of system logs.
    *   **Event Data**: JSON Log Entry

### Protected Routes (Worker Authentication Required)
These routes require valid signatures in Cloud Mode.

*   `POST /projects`
    *   **Description**: Create/Register a new project.
    *   **Request**: `application/json`
        ```json
        {
            "id": "project-xyz",
            "config": { ... } // Optional initial config
        }
        ```

*   `POST /projects/:id/deploy`
    *   **Description**: Trigger a deployment.
    *   **Request**: `application/json`
        ```json
        {
            "url": "https://bucket/artifact.zip",
            "environment": {
                "DATABASE_URL": "postgres://..."
            }
        }
        ```
    *   **Response**: `200 OK` "Deployed artifact: artifact.zip"

*   `GET /projects/:id/stats/stream` **(SSE)**
    *   **Description**: Real-time status of a specific project service.
    *   **Event Data**:
        ```json
        {
            "status": "active", 
            "active": true
        }
        ```

*   `POST /sync`
    *   **Description**: Force state synchronization.
    *   **Request**: Empty body (POST)
    *   **Response**: `application/json`
        ```json
        {
            "status": "online",
            "version": "0.1.0",
            "stats": { ... } // Global stats snapshot
        }
        ```

## 2. Service Mesh Proxy

*   `*` (Wildcard Fallback)
    *   **Logic**: Routes based on `Host` header (e.g., `app-xyz.zexio.dev`).
    *   **Port**: 8082 (Default)
