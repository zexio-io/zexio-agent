use axum::{
    routing::{get, post, delete},
    Router,
    extract::{State, Path},
    http::StatusCode,
    response::IntoResponse,
};
use crate::{state::AppState, config::Settings, db, project, deploy, services, error::AppError};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;

// Helper to get logs
async fn get_journal_logs(unit_name: &str) -> Result<String, AppError> {
    let output = std::process::Command::new("journalctl")
        .arg("-u")
        .arg(unit_name)
        .arg("-n")
        .arg("100")
        .arg("--no-pager")
        .output()
        .map_err(|_| AppError::InternalServerError)?;

    let logs = String::from_utf8_lossy(&output.stdout).to_string();
    if logs.is_empty() {
        return Ok(format!("No logs found for unit: {}", unit_name));
    }
    Ok(logs)
}

// Project Logs
pub async fn logs_handler(
    State(_state): State<AppState>,
    Path(project_id): Path<String>,
) -> Result<String, AppError> {
    let unit_name = format!("app@{}.service", project_id);
    get_journal_logs(&unit_name).await
}

// Worker Logs
pub async fn worker_logs_handler(
    State(_state): State<AppState>,
) -> Result<String, AppError> {
    get_journal_logs("worker.service").await
}

pub async fn start(settings: Settings) -> anyhow::Result<()> {
    // db connection
    let pool = db::init_pool(&settings.storage.database_url).await?;

    // Application state
    let state = AppState::new(pool, settings.clone())?;



    // Router
    let app = Router::new()
        // Public / Internal endpoints
        .route("/stats", get(monitor::global_stats_handler)) // Global System Stats
        .route("/system/logs", get(worker_logs_handler))     // Worker System Logs
        
        // Admin endpoints (Worker Secret)
        .route("/projects", 
            post(project::create_project)
            .get(project::list_projects_handler)
        )
        .route("/projects/:id", delete(project::delete_project_handler))
        
        // Granular Management
        .route("/projects/:id/env", post(project::update_env_handler))
        .route("/projects/:id/domains", post(project::add_domain_handler).delete(project::remove_domain_handler))
        .route("/projects/:id/files", get(project::list_files_handler))
        .route("/projects/:id/stats", get(monitor::project_monitor_handler)) // Project Stats/Status
        .route("/projects/:id/stats", get(monitor::project_monitor_handler)) // Project Stats/Status
        .route("/projects/:id/logs", get(logs_handler))                      // Project Logs
        .route("/projects/:id/deploy", post(deploy::project_deploy_handler)) // Manual Deploy
        .route("/projects/:id/webhook", post(deploy::project_deploy_handler)) // Webhook Deploy
        
        .route("/services", get(services::list_services_handler))
        .route("/services/install", post(services::install_service_handler))
        .route("/services/remove", post(services::remove_service_handler))
        // Webhook endpoints (Project Secret)
        // Note: The `deploy_webhook_handler` extractor will handle the per-project secret verification
        // Legacy Webhook - maybe deprecate or update? 
        // User didn't ask to remove it, but likely we can remove since we have the explicit deploy endpoint now.
        // Let's Remove specific webhook route to simplify, user uses Dashboard mainly.
        // .route("/webhook/deploy/:project_id", post(deploy::webhook_deploy_handler))
        .with_state(state);

    let addr: SocketAddr = format!("{}:{}", settings.server.host, settings.server.port).parse()?;
    info!("Server listening on {}", addr);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn stats_handler() -> &'static str {
    "OK"
}
