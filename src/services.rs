use axum::{
    extract::{State, Json},
    response::IntoResponse,
    http::StatusCode,
};
use serde::Deserialize;
use std::process::Command;
use crate::{state::AppState, errors::AppError};
use tracing::{info, error};

#[derive(Deserialize)]
pub struct InstallServiceRequest {
    pub service: String, // "postgres", "redis", "nodejs"
}

pub async fn install_service_handler(
    State(_state): State<AppState>,
    Json(payload): Json<InstallServiceRequest>,
) -> Result<impl IntoResponse, AppError> {
    
    let service = payload.service.as_str();
    info!("Request to install service: {}", service);

    let mut cmd = Command::new("bash");
    cmd.arg("-c");

    // Important: We are running apt-get/curl | bash commands here.
    // The worker MUST run as root or have passwordless sudo for specific commands.
    // Since `install.sh` and `DEPLOYMENT.md` setup the user `worker`, `apt-get` will fail 
    // unless `worker` has sudo rights.
    // Recommendation: allow `worker` to run specific install scripts or use `sudo -n`.
    
    // Command string logic
    let script = match service {
        "nodejs" => r#"
            if ! command -v node &> /dev/null; then
                curl -fsSL https://deb.nodesource.com/setup_lts.x | sudo -E bash - && \
                sudo -E apt-get install -y nodejs && \
                sudo -E npm install -g yarn pnpm pm2
            else
                echo "already installed"
            fi
        "#,
        "postgres" => r#"
            if ! command -v psql &> /dev/null; then
                sudo -E apt-get install -y postgresql postgresql-contrib && \
                sudo -E systemctl enable --now postgresql
            else
                echo "already installed"
            fi
        "#,
        "redis" => r#"
            if ! command -v redis-server &> /dev/null; then
                sudo -E apt-get install -y redis-server && \
                sudo -E sed -i 's/^supervised no/supervised systemd/' /etc/redis/redis.conf && \
                sudo -E systemctl restart redis.service && \
                sudo -E systemctl enable --now redis-server
            else
                echo "already installed"
            fi
        "#,
        _ => return Err(AppError::BadRequest("Unknown service".into())),
    };

    // Execute
    // Note: We use `sudo -E` assuming the worker user has sudo access to apt-get/systemctl without password.
    // This is a requirement for this feature to work safely.
    let output = cmd.arg(script).output().map_err(|_e| AppError::InternalServerError)?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        error!("Installation failed: {}", stderr);
        return Err(AppError::InternalServerError); // Simplify error for security? Or return log?
        // AppError::BadRequest(format!("Install failed: {}", stderr).into())
    }

    info!("Service {} installed successfully", service);

    Ok((StatusCode::OK, format!("Service {} installed", service)))
}
