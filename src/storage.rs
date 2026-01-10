use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tokio::fs;
use anyhow::{Context, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub id: String,
    pub domains: Vec<String>,
    pub encrypted_env: String, // Hex-encoded encrypted blob
    pub webhook_secret: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub struct ProjectStore {
    base_dir: PathBuf,
}

impl ProjectStore {
    pub fn new(base_dir: impl AsRef<Path>) -> Self {
        Self {
            base_dir: base_dir.as_ref().to_path_buf(),
        }
    }

    fn config_path(&self, project_id: &str) -> PathBuf {
        self.base_dir.join(project_id).join("config.json")
    }

    pub async fn create(&self, config: ProjectConfig) -> Result<()> {
        let config_path = self.config_path(&config.id);
        
        // Create project directory
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent).await
                .context("Failed to create project directory")?;
        }

        // Write config
        let json = serde_json::to_string_pretty(&config)?;
        fs::write(&config_path, json).await
            .context("Failed to write project config")?;

        Ok(())
    }

    pub async fn read(&self, project_id: &str) -> Result<ProjectConfig> {
        let config_path = self.config_path(project_id);
        let json = fs::read_to_string(&config_path).await
            .context("Failed to read project config")?;
        
        let config: ProjectConfig = serde_json::from_str(&json)
            .context("Failed to parse project config")?;
        
        Ok(config)
    }

    pub async fn update(&self, config: &ProjectConfig) -> Result<()> {
        let config_path = self.config_path(&config.id);
        let json = serde_json::to_string_pretty(config)?;
        fs::write(&config_path, json).await
            .context("Failed to update project config")?;
        
        Ok(())
    }

    pub async fn delete(&self, project_id: &str) -> Result<()> {
        let project_dir = self.base_dir.join(project_id);
        if project_dir.exists() {
            fs::remove_dir_all(&project_dir).await
                .context("Failed to delete project directory")?;
        }
        Ok(())
    }

    pub async fn list(&self) -> Result<Vec<ProjectConfig>> {
        let mut configs = Vec::new();
        
        let mut entries = fs::read_dir(&self.base_dir).await
            .context("Failed to read projects directory")?;

        while let Some(entry) = entries.next_entry().await? {
            if entry.file_type().await?.is_dir() {
                let project_id = entry.file_name().to_string_lossy().to_string();
                if let Ok(config) = self.read(&project_id).await {
                    configs.push(config);
                }
            }
        }

        Ok(configs)
    }

    pub async fn exists(&self, project_id: &str) -> bool {
        self.config_path(project_id).exists()
    }
}
