# Zexio Agent Changelog

## Version 0.4.0 - January 16, 2026

### 🚀 New Features

#### Modern CLI Interface
- **`zexio up <port>`** - Instant tunnel to local port
  - Replaced `TUNNEL_PORT` env var with direct CLI argument
  - Uses `clap` for robust argument parsing
  
- **`zexio login`** - Interactive authentication
  - Token prompt with validation (`zxp_...` format)
  - Secure identity storage (`~/.zexio/identity.json`, 0600 perms)
  - Re-authentication support
  
- **`zexio unregister`** - Clean cloud disconnect
  - Removes local identity
  - Deregisters from Zexio Cloud

#### Reliability Improvements
- **Auto-reconnect** with exponential backoff
  - Automatically reconnects if Relay restarts
  - Prevents connection spam with backoff delays

---

### 🔧 Technical Changes

**Modified Files:**
- `core/src/main.rs` - Added `clap` CLI with subcommands
- `core/src/server.rs` - Accept optional `tunnel_port` parameter
- `core/src/mesh/tunnel.rs` - Robust reconnection loop
- `core/src/registration.rs` - Added `interactive_login()` function
- `core/Cargo.toml` - Added `clap` dependency

**Commits:**
```
570ad1c feat(cli): implement 'zexio login' with interactive token prompt
c2ad59f feat(cli): implement 'zexio up <port>' command with clap
778eb7c feat(core): implement robust reconnect loop with exponential backoff
```

---

### 💥 Breaking Changes

**CLI Interface Changed:**
```bash
# Old
export TUNNEL_PORT=3000
zexio

# New
zexio up 3000
```

**Environment Variables:**
- `TUNNEL_PORT` - ❌ Removed (use CLI argument)
- `ZEXIO_CLOUD__TOKEN` - ⚠️ Optional (use `zexio login` instead)

---

### 📦 Migration Guide

```bash
# 1. Update binary
curl -sL https://get.zexio.com/agent | bash

# 2. Login with provisioning token
zexio login
# Enter token: zxp_...

# 3. Start tunnel
zexio up 3000
```

---

### 🔒 Security

- Identity files stored with `0600` permissions (Unix)
- Token validation before registration
- Secure reconnection with authentication

---

### 📝 Usage Examples

```bash
# Login
zexio login

# Start tunnel on port 3000
zexio up 3000

# Start tunnel on port 8080
zexio up 8080

# Unregister from cloud
zexio unregister
```

---

### 🐛 Bug Fixes

- Fixed tunnel reconnection loop
- Improved error messages for authentication failures

---

### 📚 Documentation

- Updated README with CLI usage
- Added architecture diagram
- Created walkthrough artifacts

---

**Status:** ✅ Production Ready
