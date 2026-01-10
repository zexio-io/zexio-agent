use sqlx::SqlitePool;
use std::sync::Arc;
use crate::config::Settings;
use crate::crypto::Crypto;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub settings: Arc<Settings>,
    pub crypto: Arc<Crypto>,
    pub worker_secret: String,
}

impl AppState {
    pub fn new(db: SqlitePool, settings: Settings) -> anyhow::Result<Self> {
        let crypto = Crypto::new(&settings.secrets.master_key_path)?;
        
        // Resolve Worker Secret
        let worker_secret = if let Some(s) = &settings.secrets.worker_secret {
            s.clone()
        } else {
            // Try reading from file
            let path = &settings.secrets.worker_secret_path;
            std::fs::read_to_string(path)
                .map(|s| s.trim().to_string())
                .map_err(|e| anyhow::anyhow!("Failed to read worker secret from {}: {}. Set PLANE__SECRETS__WORKER_SECRET or create the file.", path, e))?
        };

        if worker_secret.is_empty() {
             return Err(anyhow::anyhow!("Worker secret is empty"));
        }

        Ok(Self {
            db,
            settings: Arc::new(settings),
            crypto: Arc::new(crypto),
            worker_secret,
        })
    }
}
