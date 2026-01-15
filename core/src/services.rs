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
    info!("Request to install service: {}", service);

    match install_package(service).await {
        Ok(cmd_executed) => {
            info!(
                "Service {} installed successfully via: {}",
                service, cmd_executed
            );
            Ok((
                StatusCode::OK,
                Json(ServiceResponse {
                    status: "installed".to_string(),
                    service: service.to_string(),
                    command: cmd_executed,
                }),
            ))
        }
        Err(e) => {
            error!("Installation failed: {}", e);
            Err(AppError::InternalServerError)
        }
    }
}

async fn install_package(service: &str) -> Result<String, String> {
    #[cfg(target_os = "linux")]
    {
        install_linux(service).await
    }

    #[cfg(target_os = "macos")]
    {
        install_macos(service).await
    }

    #[cfg(target_os = "windows")]
    {
        install_windows(service).await
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        Err("Unsupported operating system".to_string())
    }
}

#[cfg(target_os = "linux")]
async fn install_linux(service: &str) -> Result<String, String> {
    let script = match service {
        "nodejs" => {
            r#"
            if ! command -v node &> /dev/null; then
                curl -fsSL https://deb.nodesource.com/setup_lts.x | sudo -E bash - && \
                sudo -E apt-get install -y nodejs && \
                sudo -E npm install -g yarn pnpm pm2
            else
                echo "already installed"
            fi
        "#
        }
        "postgres" => {
            r#"
            if ! command -v psql &> /dev/null; then
                sudo -E apt-get install -y postgresql postgresql-contrib && \
                sudo -E systemctl enable --now postgresql
            else
                echo "already installed"
            fi
        "#
        }
        "redis" => {
            r#"
            if ! command -v redis-server &> /dev/null; then
                sudo -E apt-get install -y redis-server && \
                sudo -E sed -i 's/^supervised no/supervised systemd/' /etc/redis/redis.conf && \
                sudo -E systemctl restart redis.service && \
                sudo -E systemctl enable --now redis-server
            else
                echo "already installed"
            fi
        "#
        }
        name => {
            if !name.chars().all(|c| c.is_alphanumeric() || c == '-') {
                return Err("Invalid service name".to_string());
            }
            return install_linux_generic(name).await;
        }
    };

    let output = Command::new("bash")
        .arg("-c")
        .arg(script)
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    // Return a descriptive string for complex scripts
    Ok(format!("bash script execution for {}", service))
}

#[cfg(target_os = "linux")]
async fn install_linux_generic(service: &str) -> Result<String, String> {
    let script = format!("sudo -E apt-get install -y {}", service);
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
async fn install_macos(service: &str) -> Result<String, String> {
    if !service.chars().all(|c| c.is_alphanumeric() || c == '-') {
        return Err("Invalid service name".to_string());
    }

    let cmd_str = format!("brew install {}", service);
    let output = Command::new("brew")
        .arg("install")
        .arg(service)
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }
    Ok(cmd_str)
}

#[cfg(target_os = "windows")]
async fn install_windows(service: &str) -> Result<String, String> {
    if !service.chars().all(|c| c.is_alphanumeric() || c == '-') {
        return Err("Invalid service name".to_string());
    }

    let cmd_str = format!("choco install {} -y", service);
    let output = Command::new("choco")
        .arg("install")
        .arg(service)
        .arg("-y")
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }
    Ok(cmd_str)
}

#[derive(Deserialize)]
pub struct UninstallServiceRequest {
    pub service: String,
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
