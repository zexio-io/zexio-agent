use crate::{config::Settings, crypto::Crypto, storage::ProjectStore};
use anyhow::Result;
use std::fs;

#[derive(Clone)]
pub struct AppState {
    pub store: ProjectStore,
    pub settings: Settings,
    pub crypto: Crypto,
    pub worker_secret: String,
}

impl AppState {
    pub fn new(settings: Settings) -> Result<Self> {
        // Initialize crypto
        let crypto = Crypto::new(&settings.secrets.master_key_path)?;

        // Load worker secret
        let worker_secret = fs::read_to_string(&settings.secrets.worker_secret_path)?
            .trim()
            .to_string();

        // Initialize project store
        let store = ProjectStore::new(&settings.storage.projects_dir);

        Ok(Self {
            store,
            settings,
            crypto,
            worker_secret,
        })
    }
}
