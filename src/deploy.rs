use axum::{
    extract::{State, Path},
    response::IntoResponse,
    http::StatusCode,
};
use serde::Deserialize;
use std::fs;
use std::process::Command;
use std::io::Write; // For file writing if needed
use crate::{state::AppState, errors::AppError, auth::WorkerAuth};
use tracing::{info, error};

#[derive(Deserialize)]
pub struct DeployProjectRequest {
    pub url: Option<String>,
    pub file: Option<String>,
}

pub async fn project_deploy_handler(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
    WorkerAuth(bytes): WorkerAuth,
) -> Result<impl IntoResponse, AppError> {
    // Parse Payload from Bytes (since WorkerAuth consumed body)
    let req: DeployProjectRequest = serde_json::from_slice(&bytes)
        .map_err(|e| AppError::BadRequest(format!("Invalid JSON: {}", e)))?;

    info!("Deploying project {}", project_id);
    
    let base_project_dir = format!("{}/{}", state.settings.storage.projects_dir, project_id);
    let project_dir = format!("{}/bundle", base_project_dir);
    
    // Ensure directory exists
    tokio::fs::create_dir_all(&project_dir).await.map_err(|e| AppError::InternalServerError)?;

    let artifact_name: String;

    // 1. Determine Source
    if let Some(url) = req.url {
        info!("Downloading artifact for {} from {}", project_id, url);
        // Try to derive filename
        let filename = url.split('/').last().unwrap_or("artifact.zip").to_string();
        // Fallback if empty or obscure
        let filename = if filename.is_empty() || !filename.contains('.') { 
            format!("artifact_{}.zip", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs())
        } else {
            filename
        };

        // Download
        let response = reqwest::get(&url).await
            .map_err(|e| AppError::BadRequest(format!("Failed to download: {}", e)))?;
        
        if !response.status().is_success() {
             return Err(AppError::BadRequest(format!("Download failed with status: {}", response.status())));
        }

        let bytes = response.bytes().await
            .map_err(|e| AppError::InternalServerError)?;

        let save_path = format!("{}/{}", project_dir, filename);
        tokio::fs::write(&save_path, &bytes).await.map_err(|e| AppError::InternalServerError)?;
        
        artifact_name = filename;
    } else if let Some(file) = req.file {
        info!("Deploying existing artifact {} for {}", file, project_id);
        let path = format!("{}/{}", project_dir, file);
        if tokio::fs::metadata(&path).await.is_err() {
             return Err(AppError::BadRequest(format!("File {} not found in project bundle", file)));
        }
        artifact_name = file;
    } else {
        return Err(AppError::BadRequest("Either 'url' or 'file' must be provided in payload".into()));
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
        tokio::fs::copy(&artifact_path, &app_path).await.map_err(|_| AppError::InternalServerError)?;
        // Chmod +x
        let _ = Command::new("chmod").arg("+x").arg(&app_path).output();
    }

    // 3. Decrypt Environment from JSON config
    let config = state.store.read(&project_id).await
        .map_err(|_| AppError::BadRequest("Project not found".into()))?;

    if !config.encrypted_env.is_empty() {
        // Decode hex string to bytes
        let enc_env = hex::decode(&config.encrypted_env)
            .map_err(|_| AppError::InternalServerError)?;
        
        let env_bytes = state.crypto.decrypt(&enc_env)
            .map_err(|_| AppError::InternalServerError)?; 
        let env_str = String::from_utf8(env_bytes)
             .map_err(|_| AppError::InternalServerError)?;
        
        let env_path = format!("{}/.env", project_dir);
        tokio::fs::write(&env_path, env_str).await.map_err(|_| AppError::InternalServerError)?;
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

    Ok((StatusCode::OK, format!("Deployed artifact: {}", artifact_name)))
}
