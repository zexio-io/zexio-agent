use config::{Config, ConfigError};
use serde::Deserialize;
use std::env;
use std::path::PathBuf;

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

/// Get OS-specific config directory
fn get_config_dir() -> PathBuf {
    if cfg!(target_os = "windows") {
        // Windows: C:\ProgramData\Zexio
        PathBuf::from(env::var("PROGRAMDATA").unwrap_or_else(|_| "C:\\ProgramData".to_string()))
            .join("Zexio")
    } else if cfg!(target_os = "macos") {
        // macOS: /Library/Application Support/Zexio
        PathBuf::from("/Library/Application Support/Zexio")
    } else {
        // Linux: /etc/zexio
        PathBuf::from("/etc/zexio")
    }
}

/// Get OS-specific data directory
fn get_data_dir() -> PathBuf {
    if cfg!(target_os = "windows") {
        // Windows: C:\ProgramData\Zexio\data
        PathBuf::from(env::var("PROGRAMDATA").unwrap_or_else(|_| "C:\\ProgramData".to_string()))
            .join("Zexio")
            .join("data")
    } else if cfg!(target_os = "macos") {
        // macOS: /Library/Application Support/Zexio/data
        PathBuf::from("/Library/Application Support/Zexio/data")
    } else {
        // Linux: /var/lib/zexio
        PathBuf::from("/var/lib/zexio")
    }
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

        // Get OS-specific paths
        let config_dir = get_config_dir();
        let data_dir = get_data_dir();

        let s = Config::builder()
            // Start with default values
            .set_default("server.port", port as i64)?
            .set_default("server.mesh_port", mesh_port as i64)?
            .set_default("server.host", "0.0.0.0")?
            .set_default("server.public_hostname", None::<String>)?
            .set_default("server.public_ip", None::<String>)?
            
            // Default Storage Paths (OS-specific)
            .set_default("storage.projects_dir", data_dir.join("apps").to_string_lossy().to_string())?
            
            // Default Secrets Paths (OS-specific)
            .set_default("secrets.worker_secret_path", config_dir.join("worker.secret").to_string_lossy().to_string())?
            .set_default("secrets.master_key_path", config_dir.join("master.key").to_string_lossy().to_string())?
            .set_default("secrets.identity_path", config_dir.join("identity.json").to_string_lossy().to_string())?
            .set_default("secrets.provisioning_token_path", config_dir.join("provisioning_token").to_string_lossy().to_string())?

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
