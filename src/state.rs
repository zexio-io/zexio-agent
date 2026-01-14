use crate::{config::Settings, crypto::Crypto, storage::ProjectStore};
use anyhow::Result;
use std::fs;

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
        // Initialize crypto
        let crypto = Crypto::new(&settings.secrets.master_key_path)?;

        // Load worker secret (Graceful fallback for Standalone Mode)
        let worker_secret = match fs::read_to_string(&settings.secrets.worker_secret_path) {
            Ok(s) => s.trim().to_string(),
            Err(_) => {
                println!("⚠️  Running in Standalone Mode (No Cloud Identity)");
                "standalone-mode".to_string()
            }
        };

        // Initialize Redis
        let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());
        let redis = redis::Client::open(redis_url)?;

        // Mesh JWT Secret
        let mesh_jwt_secret = std::env::var("MESH_JWT_SECRET").unwrap_or_else(|_| "zexio-mesh-secret-key".to_string());

        // Initialize Plugin Manager


        Ok(Self {
            store: crate::storage::ProjectStore::new(&settings.storage.projects_dir),
            settings,
            crypto,
            worker_secret,
            redis,
            mesh_jwt_secret,

        })
    }
}
