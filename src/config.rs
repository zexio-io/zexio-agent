use config::{Config, ConfigError, File, Environment};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: ServerSettings,
    pub storage: StorageSettings,
    pub caddy: CaddySettings,
    pub secrets: SecretsSettings,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerSettings {
    pub port: u16,
    pub host: String, // Internal bind host
    pub public_hostname: Option<String>,
    pub public_ip: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct StorageSettings {
    pub projects_dir: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CaddySettings {
    pub admin_api: String,
    pub caddyfile_path: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SecretsSettings {
    pub worker_secret: Option<String>,
    pub worker_secret_path: String,
    pub master_key_path: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let env = env::var("RUN_MODE").unwrap_or_else(|_| "production".into());

        let s = Config::builder()
            // Start with default values
            .set_default("server.port", 3000)?
            .set_default("server.host", "0.0.0.0")?
            .set_default("server.public_hostname", None::<String>)?
            .set_default("server.public_ip", None::<String>)?
            
            // Default Storage Paths (Production)
            .set_default("storage.projects_dir", "/apps")?
            
            // Default Caddy Config
            .set_default("caddy.admin_api", "http://localhost:2019")?
            .set_default("caddy.caddyfile_path", "/etc/caddy/Caddyfile")?
            
            // Default Secret Paths (Production)
            .set_default("secrets.master_key_path", "/etc/vectis/master.key")?
            .set_default("secrets.worker_secret_path", "/etc/vectis/worker.secret")?
            
            // Merge valid configuration files
            .add_source(File::with_name("config/default").required(false))
            .add_source(File::with_name(&format!("config/{}", env)).required(false))
            .add_source(File::with_name("/etc/vectis/config").required(false))
            // Merge environment variables
            .add_source(Environment::with_prefix("PLANE").separator("__"))
            // Config from specific ENV vars for simplicity if user ignores "PLANE__" prefix convention for these two
            // But let's stick to standard `add_source(Environment)` which maps `PLANE__SERVER__PORT` to `server.port`.
            .build()?;

        s.try_deserialize()
    }
}
