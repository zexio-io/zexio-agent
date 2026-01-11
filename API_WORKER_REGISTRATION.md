# Worker Registration API

## Endpoint

```
POST /api/v1/workers
```

## Authentication

```
Authorization: Bearer {VECTIS_TOKEN}
```

## Request Body

```typescript
{
  ip: string;      // Worker public IP (auto-detected)
  secret: string;  // HMAC authentication secret
}
```

**Example:**
```json
{
  "ip": "203.0.113.5",
  "secret": "a1b2c3d4e5f6..."
}
```

## Response

**Success (201 Created):**
```typescript
{
  id: string;        // Worker ID (e.g., "wrk_abc123")
  subdomain: string; // Assigned subdomain (e.g., "worker-a1b2c3.vectis.dev")
  ip: string;        // Confirmed IP address
}
```

**Example:**
```json
{
  "id": "wrk_abc123",
  "subdomain": "worker-a1b2c3.vectis.dev",
  "ip": "203.0.113.5"
}
```

**Error (400 Bad Request):**
```json
{
  "error": "Invalid request",
  "message": "IP address is required"
}
```

**Error (401 Unauthorized):**
```json
{
  "error": "Unauthorized",
  "message": "Invalid or missing token"
}
```

**Error (409 Conflict):**
```json
{
  "error": "Worker already exists",
  "message": "A worker with this IP is already registered"
}
```

## Implementation Notes

### Backend (Dashboard)

1. **Validate token** - Check `Authorization` header
2. **Generate worker ID** - Create unique ID (e.g., `wrk_` + nanoid)
3. **Generate subdomain** - Create unique subdomain (e.g., `worker-` + short hash)
4. **Save to database:**
   ```sql
   INSERT INTO workers (id, subdomain, ip, secret, created_at)
   VALUES ($1, $2, $3, $4, NOW())
   ```
5. **Return response** with ID, subdomain, and IP

### Worker (install.sh)

1. **Auto-detect IP** - `curl -s https://api.ipify.org`
2. **Read secret** - From `/etc/vectis/worker.secret`
3. **Send registration** - POST to dashboard
4. **Save response** - To `/etc/vectis/worker.conf`:
   ```bash
   WORKER_ID=wrk_abc123
   SUBDOMAIN=worker-a1b2c3.vectis.dev
   PUBLIC_IP=203.0.113.5
   SYNC_STATUS=synced
   ```

## Database Schema

```sql
CREATE TABLE workers (
  id VARCHAR(50) PRIMARY KEY,
  subdomain VARCHAR(100) UNIQUE NOT NULL,
  ip VARCHAR(45) NOT NULL,
  secret TEXT NOT NULL,
  status VARCHAR(20) DEFAULT 'active',
  created_at TIMESTAMP DEFAULT NOW(),
  updated_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX idx_workers_ip ON workers(ip);
CREATE INDEX idx_workers_subdomain ON workers(subdomain);
```

## Security

- ✅ Token-based authentication
- ✅ HMAC secret for worker auth
- ✅ IP validation
- ✅ Unique subdomain generation
- ✅ Rate limiting recommended
