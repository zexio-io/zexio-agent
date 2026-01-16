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
    #[allow(dead_code)]
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
    // Check if running as root or with write permissions
    let use_system_paths = is_root_or_has_system_access();

    if cfg!(target_os = "windows") {
        // Windows: C:\ProgramData\Zexio
        PathBuf::from(env::var("PROGRAMDATA").unwrap_or_else(|_| "C:\\ProgramData".to_string()))
            .join("Zexio")
    } else if cfg!(target_os = "macos") {
        if use_system_paths {
            // macOS (system): /Library/Application Support/Zexio
            PathBuf::from("/Library/Application Support/Zexio")
        } else {
            // macOS (user): ~/Library/Application Support/Zexio
            dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("Library")
                .join("Application Support")
                .join("Zexio")
        }
    } else {
        // Linux (including Raspberry Pi)
        if use_system_paths {
            // Linux (system): /etc/zexio
            PathBuf::from("/etc/zexio")
        } else {
            // Linux (user/IoT): ~/.config/zexio
            dirs::config_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("zexio")
        }
    }
}

/// Get OS-specific data directory
fn get_data_dir() -> PathBuf {
    let use_system_paths = is_root_or_has_system_access();

    if cfg!(target_os = "windows") {
        // Windows: C:\ProgramData\Zexio\data
        PathBuf::from(env::var("PROGRAMDATA").unwrap_or_else(|_| "C:\\ProgramData".to_string()))
            .join("Zexio")
            .join("data")
    } else if cfg!(target_os = "macos") {
        if use_system_paths {
            // macOS (system): /Library/Application Support/Zexio/data
            PathBuf::from("/Library/Application Support/Zexio/data")
        } else {
            // macOS (user): ~/Library/Application Support/Zexio/data
            dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("Library")
                .join("Application Support")
                .join("Zexio")
                .join("data")
        }
    } else {
        // Linux (including Raspberry Pi)
        if use_system_paths {
            // Linux (system): /var/lib/zexio
            PathBuf::from("/var/lib/zexio")
        } else {
            // Linux (user/IoT): ~/.local/share/zexio
            dirs::data_local_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("zexio")
        }
    }
}

/// Check if running as root or has system-level access
fn is_root_or_has_system_access() -> bool {
    #[cfg(unix)]
    {
        // On Unix (Linux/macOS), check if UID is 0 (root)
        unsafe { libc::geteuid() == 0 }
    }

    #[cfg(not(unix))]
    {
        // On Windows, assume system access (can be enhanced with Windows API)
        true
    }
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let env = env::var("RUN_MODE").unwrap_or_else(|_| "production".into());

        // Allow simple SERVER_PORT override
        let port = env::var("SERVER_PORT")
            .ok()
            .and_then(|p| p.parse::<u16>().ok())
            .unwrap_or(8081); // Changed from 3000 to avoid conflict with GUI dev server

        let mesh_port = env::var("MESH_PORT")
            .ok()
            .and_then(|p| p.parse::<u16>().ok())
            .unwrap_or(8082); // Changed from 8080 to avoid common conflicts

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
            .set_default(
                "storage.projects_dir",
                data_dir.join("apps").to_string_lossy().to_string(),
            )?
            // Default Secrets Paths (OS-specific)
            .set_default(
                "secrets.worker_secret_path",
                config_dir
                    .join("worker.secret")
                    .to_string_lossy()
                    .to_string(),
            )?
            .set_default(
                "secrets.master_key_path",
                config_dir.join("master.key").to_string_lossy().to_string(),
            )?
            .set_default(
                "secrets.identity_path",
                config_dir
                    .join("identity.json")
                    .to_string_lossy()
                    .to_string(),
            )?
            .set_default(
                "secrets.provisioning_token_path",
                config_dir
                    .join("provisioning_token")
                    .to_string_lossy()
                    .to_string(),
            )?
            // Default Cloud Settings
            .set_default("cloud.api_url", "https://api.zexio.io")?
            .set_default("debug", false)?
            // Load config file if exists
            .add_source(config::File::with_name(&format!("config/{}", env)).required(false))
            .add_source(
                config::Environment::with_prefix("ZEXIO").separator("__"), // ZEXIO_CLOUD__API_URL overrides cloud.api_url
            )
            .build()?;

        s.try_deserialize()
    }
}
