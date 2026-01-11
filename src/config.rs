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

        // Allow simple SERVER_PORT override
        let port = env::var("SERVER_PORT")
            .ok()
            .and_then(|p| p.parse::<u16>().ok())
            .unwrap_or(3000);

        let s = Config::builder()
            // Start with default values
            .set_default("server.port", port as i64)?
            .set_default("server.host", "0.0.0.0")?
            .set_default("server.public_hostname", None::<String>)?
            .set_default("server.public_ip", None::<String>)?
            
            // Default Storage Paths (Production)
            .set_default("storage.projects_dir", "/vectis/apps")?
            .set_default("storage.config_dir", "/etc/vectis")?
            
            // Default Caddy Settings
            .set_default("caddy.admin_api", "http://localhost:2019")?
            .set_default("caddy.caddyfile_path", "/etc/caddy/Caddyfile")?
            
            // Default Secrets Paths
            .set_default("secrets.worker_secret_path", "/etc/vectis/worker.secret")?
            .set_default("secrets.master_key_path", "/etc/vectis/master.key")?
            
            // Load config file if exists
            .add_source(
                config::File::with_name(&format!("config/{}", env))
                    .required(false)
            )
            .build()?;

        s.try_deserialize()
    }
}
