use crate::{errors::AppError, state::AppState};
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::process::Command;
use tracing::{error, info};

#[derive(Deserialize)]
pub struct InstallServiceRequest {
    pub service: String,
    pub command: Option<String>,
}

#[derive(Serialize)]
pub struct ServiceResponse {
    pub status: String,
    pub service: String,
    pub command: String,
}

pub async fn install_service_handler(
    State(_state): State<AppState>,
    Json(payload): Json<InstallServiceRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = payload.service.as_str();
    
    // If a command is provided in the request, use it. 
    // Otherwise, we might eventually pull it from a catalog, but for now we require it or fail.
    let cmd_to_run = payload.command.clone().ok_or_else(|| {
        error!("No command provided for service installation: {}", service);
        AppError::BadRequest
    })?;

    info!("Request to run command for service {}: {}", service, cmd_to_run);

    match run_generic_command(&cmd_to_run).await {
        Ok(_) => {
            info!("Command executed successfully for {}", service);
            Ok((
                StatusCode::OK,
                Json(ServiceResponse {
                    status: "executed".to_string(),
                    service: service.to_string(),
                    command: cmd_to_run,
                }),
            ))
        }
        Err(e) => {
            error!("Execution failed: {}", e);
            Err(AppError::InternalServerError)
        }
    }
}

pub async fn run_generic_command(cmd: &str) -> Result<String, String> {
    // Safety check: Filter out potentially destructive commands
    if is_dangerous_command(cmd) {
        error!("🚨 BLOCKED: Potential destructive command detected: {}", cmd);
        return Err("Command blocked: Potential system destruction detected for safety reasons.".to_string());
    }

    let output = if cfg!(target_os = "windows") {
        Command::new("powershell")
            .arg("-Command")
            .arg(cmd)
            .output()
            .map_err(|e| e.to_string())?
    } else {
        Command::new("bash")
            .arg("-c")
            .arg(cmd)
            .output()
            .map_err(|e| e.to_string())?
    };

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Simple safety filter to catch accidental or malicious system destruction
fn is_dangerous_command(cmd: &str) -> bool {
    let cmd_lower = cmd.to_lowercase();
    
    // Patterns that are highly likely to cause irreversible system damage
    // Note: This is a guardrail, not an absolute sandbox.
    let extreme_danger = [
        "rm -rf / ",
        "rm -rf /\"",
        "rm -rf /*",
        "rm -rf /etc",
        "rm -rf /bin",
        "rm -rf /boot",
        "rm -rf /dev",
        "rm -rf /sbin",
        "rm -rf /usr",
        "mkfs",
        "dd if=/dev/",
        "> /dev/sd",
        "> /dev/nvme",
        "chmod -r 777 /",
        "chown -r 777 /",
    ];

    extreme_danger.iter().any(|&p| cmd_lower.contains(p))
}

#[derive(Deserialize)]
pub struct UninstallServiceRequest {
    pub service: String,
    pub command: Option<String>,
}

pub async fn uninstall_service_handler(
    State(_state): State<AppState>,
    Json(payload): Json<UninstallServiceRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = payload.service.as_str();
    info!("Request to UNINSTALL service: {}", service);

    match uninstall_package(service).await {
        Ok(cmd_executed) => {
            info!(
                "Service {} uninstalled successfully via: {}",
                service, cmd_executed
            );
            Ok((
                StatusCode::OK,
                Json(ServiceResponse {
                    status: "uninstalled".to_string(),
                    service: service.to_string(),
                    command: cmd_executed,
                }),
            ))
        }
        Err(e) => {
            error!("Uninstallation failed: {}", e);
            Err(AppError::InternalServerError)
        }
    }
}

async fn uninstall_package(service: &str) -> Result<String, String> {
    #[cfg(target_os = "linux")]
    {
        uninstall_linux(service).await
    }

    #[cfg(target_os = "macos")]
    {
        uninstall_macos(service).await
    }

    #[cfg(target_os = "windows")]
    {
        uninstall_windows(service).await
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        Err("Unsupported operating system".to_string())
    }
}

#[cfg(target_os = "linux")]
async fn uninstall_linux(service: &str) -> Result<String, String> {
    if !service.chars().all(|c| c.is_alphanumeric() || c == '-') {
        return Err("Invalid service name".to_string());
    }

    let script = format!("sudo -E apt-get remove -y {}", service);
    let output = Command::new("bash")
        .arg("-c")
        .arg(&script)
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }
    Ok(script)
}

#[cfg(target_os = "macos")]
async fn uninstall_macos(service: &str) -> Result<String, String> {
    if !service.chars().all(|c| c.is_alphanumeric() || c == '-') {
        return Err("Invalid service name".to_string());
    }

    let cmd_str = format!("brew uninstall {}", service);
    let output = Command::new("brew")
        .arg("uninstall")
        .arg(service)
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }
    Ok(cmd_str)
}

#[cfg(target_os = "windows")]
async fn uninstall_windows(service: &str) -> Result<String, String> {
    if !service.chars().all(|c| c.is_alphanumeric() || c == '-') {
        return Err("Invalid service name".to_string());
    }

    let cmd_str = format!("choco uninstall {} -y", service);
    let output = Command::new("choco")
        .arg("uninstall")
        .arg(service)
        .arg("-y")
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }
    Ok(cmd_str)
}
