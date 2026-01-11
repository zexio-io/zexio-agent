# Worker API (Rust Node)

This API is exposed by the Rust Node on port 3000. Protected routes require HMAC authentication using the `worker_secret`.

## Public Routes

### Health Check
`GET /health`
Returns `OK` if the node is running.

### System Stats
`GET /stats`
Returns current system resource usage (CPU, Memory, Disk).

## Protected Routes
Requires `Authorization: Bearer <HMAC_SIGNATURE>`

### Manual Sync
`POST /sync`
Triggered by the Dashboard to verify worker status and get current metadata.

**Response:**
```json
{
  "status": "online",
  "version": "1.0.1",
  "stats": {
    "cpu_usage": 12.5,
    "memory_used": 1024,
    "memory_total": 4096,
    "memory_percent": 25.0,
    "disk_used": 50000,
    "disk_total": 100000,
    "disk_percent": 50.0
  }
}
```

### Project Management
- `POST /projects` - Create/Deploy project
- `DELETE /projects/:id` - Remove project
- `GET /projects/:id/stats` - Get project-specific stats
- `GET /projects/:id/logs` - Get project logs
