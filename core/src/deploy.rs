use crate::{errors::AppError, state::AppState};
use axum::{
    body::Bytes,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use std::process::Command;
use tracing::{error, info};

use std::collections::HashMap;

#[derive(Deserialize)]
pub struct DeployProjectRequest {
    #[serde(alias = "bundle_url")]
    pub url: Option<String>,
    pub file: Option<String>,
    pub environment: Option<HashMap<String, String>>,
}

pub async fn project_deploy_handler(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
    bytes: Bytes,
) -> Result<impl IntoResponse, AppError> {
    // Parse Payload from Bytes (since WorkerAuth consumed body)
    let req: DeployProjectRequest = serde_json::from_slice(&bytes)
        .map_err(|e| AppError::BadRequest(format!("Invalid JSON: {}", e)))?;

    info!("Deploying project {}", project_id);

    let base_project_dir = format!("{}/{}", state.settings.storage.projects_dir, project_id);
    let project_dir = format!("{}/bundle", base_project_dir);

    // Ensure directory exists
    tokio::fs::create_dir_all(&project_dir)
        .await
        .map_err(|_e| AppError::InternalServerError)?;

    let artifact_name: String;

    // 1. Determine Source
    if let Some(url) = req.url {
        info!("Downloading artifact for {} from {}", project_id, url);
        // Try to derive filename
        let filename = url.split('/').last().unwrap_or("artifact.zip").to_string();
        // Fallback if empty or obscure
        let filename = if filename.is_empty() || !filename.contains('.') {
            format!(
                "artifact_{}.zip",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            )
        } else {
            filename
        };

        // Download
        let response = reqwest::get(&url)
            .await
            .map_err(|e| AppError::BadRequest(format!("Failed to download: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::BadRequest(format!(
                "Download failed with status: {}",
                response.status()
            )));
        }

        let bytes = response
            .bytes()
            .await
            .map_err(|_e| AppError::InternalServerError)?;

        let save_path = format!("{}/{}", project_dir, filename);
        tokio::fs::write(&save_path, &bytes)
            .await
            .map_err(|_e| AppError::InternalServerError)?;

        artifact_name = filename;
    } else if let Some(file) = req.file {
        info!("Deploying existing artifact {} for {}", file, project_id);
        let path = format!("{}/{}", project_dir, file);
        if tokio::fs::metadata(&path).await.is_err() {
            return Err(AppError::BadRequest(format!(
                "File {} not found in project bundle",
                file
            )));
        }
        artifact_name = file;
    } else {
        return Err(AppError::BadRequest(
            "Either 'url' or 'file' must be provided in payload".into(),
        ));
    }

    let artifact_path = format!("{}/{}", project_dir, artifact_name);

    // 2. Extract / Setup
    // Use std::process::Command for unzip/chmod as it is blocking but simple.
    // Ideally use tokio::process::Command in async code.
    if artifact_name.ends_with(".zip") {
        // Unzip
        let output = Command::new("unzip")
            .arg("-o") // overwrite
            .arg(&artifact_path)
            .arg("-d")
            .arg(&project_dir)
            .output()
            .map_err(|_| AppError::InternalServerError)?;

        if !output.status.success() {
            let err = String::from_utf8_lossy(&output.stderr);
            error!("Unzip failed: {}", err);
            return Err(AppError::InternalServerError);
        }
    } else {
        // Assume binary. Copy to 'app'
        let app_path = format!("{}/app", project_dir);
        // Copy file
        tokio::fs::copy(&artifact_path, &app_path)
            .await
            .map_err(|_| AppError::InternalServerError)?;
        // Chmod +x
        let _ = Command::new("chmod").arg("+x").arg(&app_path).output();
    }

    // 3. Setup Environment (.env)
    let env_path = format!("{}/.env", project_dir);
    let mut env_content = String::new();

    // 3.1 Use provided environment from payload (Higher priority during deployment)
    if let Some(env_map) = req.environment {
        for (k, v) in env_map {
            env_content.push_str(&format!("{}={}\n", k, v));
        }
    }

    // 3.2 Add/Merge encrypted environment from stored config
    if let Ok(config) = state.store.read(&project_id).await {
        if !config.encrypted_env.is_empty() {
            if let Ok(enc_env) = hex::decode(&config.encrypted_env) {
                if let Ok(env_bytes) = state.crypto.decrypt(&enc_env) {
                    if let Ok(stored_env_str) = String::from_utf8(env_bytes) {
                        env_content.push_str("\n# Stored Secret Env\n");
                        env_content.push_str(&stored_env_str);
                    }
                }
            }
        }
    }

    if !env_content.is_empty() {
        tokio::fs::write(&env_path, env_content)
            .await
            .map_err(|_| AppError::InternalServerError)?;
    }

    // 4. Generate/Restart Systemd
    // Ensure Systemd knows about changes if we updated unit file (we didn't, but good practice)
    let _ = Command::new("systemctl").arg("daemon-reload").status();

    // Restart service
    let output = Command::new("systemctl")
        .arg("restart")
        .arg(format!("app@{}", project_id))
        .output()
        .map_err(|_| AppError::InternalServerError)?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        error!("Systemd restart failed: {}", stderr);
        return Err(AppError::InternalServerError);
    }

    Ok((
        StatusCode::OK,
        format!("Deployed artifact: {}", artifact_name),
    ))
}
