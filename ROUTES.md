# Plane Service Routes Analysis

## 1. Management API (HTTP & SSE)

### Public Routes (No Authentication Required)
These routes are open and do not require any authentication headers.

*   `GET /health`
    *   **Response**: `text/plain` "OK"
    *   **Description**: Health check endpoint to verify service availability.
*   `GET /stats`
    *   **Response**: `application/json`
    *   **Description**: Returns a snapshot of global worker statistics.
*   `GET /stats/stream` **(SSE)**
    *   **Response**: Server-Sent Events stream
    *   **Description**: Real-time stream of global worker statistics.
*   `GET /system/logs`
    *   **Response**: `application/json`
    *   **Description**: Returns a snapshot of the worker system logs.
*   `GET /system/logs/stream` **(SSE)**
    *   **Response**: Server-Sent Events stream
    *   **Description**: Real-time stream of worker system logs.

### Protected Routes (Worker Authentication Required)
These routes require a valid worker authentication mechanism (typically handled via middleware).

*   `POST /projects`
    *   **Description**: Create a new project.
*   `GET /projects`
    *   **Description**: List all existing projects.
*   `DELETE /projects/:id`
    *   **Description**: Delete a specific project by ID.
*   `POST /projects/:id/env`
    *   **Description**: Update environment variables for a specific project.
*   `POST /projects/:id/domains`
    *   **Description**: Add a custom domain to a project.
*   `DELETE /projects/:id/domains`
    *   **Description**: Remove a custom domain from a project.
*   `GET /projects/:id/files`
    *   **Description**: List files associated with a project.
*   `GET /projects/:id/stats`
    *   **Description**: Get a snapshot of statistics for a specific project.
*   `GET /projects/:id/stats/stream` **(SSE)**
    *   **Description**: Real-time stream of statistics for a specific project.
*   `GET /projects/:id/logs`
    *   **Description**: Get a snapshot of logs for a specific project.
*   `GET /projects/:id/logs/stream` **(SSE)**
    *   **Description**: Real-time stream of logs for a specific project.
*   `POST /projects/:id/deploy`
    *   **Description**: Trigger a manual deployment for a project.
*   `POST /projects/:id/webhook`
    *   **Description**: Webhook endpoint for automatic deployment triggers.
*   `POST /services/install`
    *   **Description**: Install additional services.
*   `POST /firewall/configure`
    *   **Description**: Configure firewall settings.
*   `POST /sync`
    *   **Description**: Force synchronization of worker state.

## 2. Service Mesh Proxy

The mesh proxy handles inter-service traffic and ingress. It does not define static routes but uses a fallback handler to route based on the `Host` header.

*   `*` (Wildcard Fallback)
    *   **Logic**: Inspects the `Host` header (e.g., `*.zexio.internal` or `*.zexio.app`) and proxies the request to the appropriate service container/worker.
    *   **Authentication**: Validates JWT via `Authorization: Bearer <token>` header to ensure tenant isolation.
    *   **Tenant Isolation**: Verifies that the `orgId` in the token matches the target service's organization.

## Summary of SSE (Server-Sent Events) Routes
Endpoints designed for real-time data streaming:
1.  `/stats/stream` (Global stats)
2.  `/system/logs/stream` (System logs)
3.  `/projects/:id/stats/stream` (Project stats)
4.  `/projects/:id/logs/stream` (Project logs)
