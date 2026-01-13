use config::{Config, ConfigError};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: ServerSettings,
    pub storage: StorageSettings,
    pub secrets: SecretsSettings,
    pub cloud: CloudSettings,
    pub debug: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerSettings {
    pub port: u16,
    pub mesh_port: u16,
    pub host: String, // Internal bind host
    pub public_hostname: Option<String>,
    pub public_ip: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct StorageSettings {
    pub projects_dir: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SecretsSettings {
    #[allow(dead_code)]
    pub worker_secret: Option<String>,
    pub start_port: Option<u16>, // Unused but kept for compatibility
    pub worker_secret_path: String,
    pub master_key_path: String,
    pub identity_path: String,
    pub provisioning_token_path: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CloudSettings {
    pub api_url: String,
    pub token: Option<String>,
    pub worker_id: Option<String>,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let env = env::var("RUN_MODE").unwrap_or_else(|_| "production".into());

        // Allow simple SERVER_PORT override
        let port = env::var("SERVER_PORT")
            .ok()
            .and_then(|p| p.parse::<u16>().ok())
            .unwrap_or(3000);

        let mesh_port = env::var("MESH_PORT")
            .ok()
            .and_then(|p| p.parse::<u16>().ok())
            .unwrap_or(8080);

        let s = Config::builder()
            // Start with default values
            .set_default("server.port", port as i64)?
            .set_default("server.mesh_port", mesh_port as i64)?
            .set_default("server.host", "0.0.0.0")?
            .set_default("server.public_hostname", None::<String>)?
            .set_default("server.public_ip", None::<String>)?
            
            // Default Storage Paths (Production)
            .set_default("storage.projects_dir", "/zexio/apps")?
            .set_default("storage.config_dir", "/etc/zexio")?
            
            // Default Secrets Paths
            .set_default("secrets.worker_secret_path", "/etc/zexio/worker.secret")?
            .set_default("secrets.master_key_path", "/etc/zexio/master.key")?
            .set_default("secrets.identity_path", "/etc/zexio/identity.json")?
            .set_default("secrets.provisioning_token_path", "/etc/zexio/provisioning_token")?

            // Default Cloud Settings
            .set_default("cloud.api_url", "https://api.zexio.io")?
            .set_default("debug", false)?
            
            // Load config file if exists
            .add_source(
                config::File::with_name(&format!("config/{}", env))
                    .required(false)
            )
            .add_source(
                config::Environment::with_prefix("ZEXIO")
                    .separator("__") // ZEXIO_CLOUD__API_URL overrides cloud.api_url
            )
            .build()?;

        s.try_deserialize()
    }
}
