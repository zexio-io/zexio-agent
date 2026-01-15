use crate::{config::Settings, crypto::Crypto, storage::ProjectStore};
use anyhow::Result;
use std::fs;
use std::path::Path;

#[derive(Clone)]
pub struct AppState {
    pub store: ProjectStore,
    pub settings: Settings,
    pub crypto: Crypto,
    pub worker_secret: String,
    pub redis: redis::Client,
    pub mesh_jwt_secret: String,
}

impl AppState {
    pub fn new(settings: Settings) -> Result<Self> {
        // Ensure all required directories exist
        Self::ensure_directories(&settings)?;

        // Initialize crypto (auto-generates master key if needed)
        let crypto = Crypto::new(&settings.secrets.master_key_path)?;

        // Load or generate worker secret
        let worker_secret =
            Self::load_or_generate_worker_secret(&settings.secrets.worker_secret_path)?;

        // Initialize Redis
        let redis_url =
            std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());
        let redis = redis::Client::open(redis_url)?;

        // Mesh JWT Secret
        let mesh_jwt_secret = std::env::var("MESH_JWT_SECRET")
            .unwrap_or_else(|_| "zexio-mesh-secret-key".to_string());

        Ok(Self {
            store: crate::storage::ProjectStore::new(&settings.storage.projects_dir),
            settings,
            crypto,
            worker_secret,
            redis,
            mesh_jwt_secret,
        })
    }

    /// Ensure all required directories exist
    fn ensure_directories(settings: &Settings) -> Result<()> {
        // Create projects directory
        if !Path::new(&settings.storage.projects_dir).exists() {
            fs::create_dir_all(&settings.storage.projects_dir)?;
            tracing::info!(
                "Created projects directory: {}",
                settings.storage.projects_dir
            );
        }

        // Create secrets directory (parent of all secret files)
        if let Some(parent) = Path::new(&settings.secrets.master_key_path).parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
                tracing::info!("Created secrets directory: {:?}", parent);
            }
        }

        Ok(())
    }

    /// Load or generate worker secret
    fn load_or_generate_worker_secret(path: &str) -> Result<String> {
        if Path::new(path).exists() {
            let secret = fs::read_to_string(path)?.trim().to_string();
            Ok(secret)
        } else {
            // Generate new worker secret
            let secret = Self::generate_secret();

            // Create parent directory if needed
            if let Some(parent) = Path::new(path).parent() {
                fs::create_dir_all(parent)?;
            }

            // Write secret to file
            fs::write(path, &secret)?;
            tracing::info!("Generated new worker secret at {}", path);

            Ok(secret)
        }
    }

    /// Generate a random secret (32 bytes as hex)
    fn generate_secret() -> String {
        use rand::Rng;
        let mut secret = [0u8; 32];
        rand::thread_rng().fill(&mut secret);
        hex::encode(secret)
    }
}
