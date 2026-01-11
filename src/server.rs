use axum::{
    routing::{get, post, delete},
    Router,
    extract::{State, Path},
};
use crate::{state::AppState, config::Settings, project, deploy, services, monitor, middleware, streams};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;
use axum::middleware as axum_middleware;


pub async fn start(settings: Settings) -> anyhow::Result<()> {
    // Application state (no database needed!)
    let state = AppState::new(settings.clone())?;

    // Protected routes (require worker authentication)
    let protected_routes = Router::new()
        .route("/projects", 
            post(project::create_project)
            .get(project::list_projects_handler)
        )
        .route("/projects/:id", delete(project::delete_project_handler))
        .route("/projects/:id/env", post(project::update_env_handler))
        .route("/projects/:id/domains", post(project::add_domain_handler).delete(project::remove_domain_handler))
        .route("/projects/:id/files", get(project::list_files_handler))
        .route("/projects/:id/stats", get(monitor::project_monitor_handler))
        .route("/projects/:id/stats/stream", get(monitor::project_monitor_stream))  // SSE!
        .route("/projects/:id/logs", get(streams::project_logs_handler))  // JSON (one-time)
        .route("/projects/:id/logs/stream", get(streams::project_logs_stream))  // SSE!
        .route("/projects/:id/deploy", post(deploy::project_deploy_handler))
        .route("/projects/:id/webhook", post(deploy::project_deploy_handler))
        .route("/services/install", post(services::install_service_handler))
        .route("/sync", post(monitor::sync_handler))
        .layer(axum_middleware::from_fn_with_state(
            state.clone(),
            middleware::worker_auth_middleware
        ));

    // Public routes (no auth required)
    let public_routes = Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/stats", get(monitor::global_stats_handler))
        .route("/stats/stream", get(monitor::global_stats_stream))  // SSE!
        .route("/system/logs", get(streams::worker_logs_handler))   // JSON (one-time)
        .route("/system/logs/stream", get(streams::worker_logs_stream));  // SSE!

    // Combine routes
    let app = public_routes
        .merge(protected_routes)
        .with_state(state.clone());

    let mgmt_addr: SocketAddr = format!("{}:{}", settings.server.host, settings.server.port).parse()?;
    let mesh_addr: SocketAddr = format!("{}:{}", settings.server.host, settings.server.mesh_port).parse()?;
    
    info!("Management API listening on {}", mgmt_addr);
    info!("Service Mesh Proxy listening on {}", mesh_addr);

    // Create Mesh Proxy Router
    let mesh_app = Router::new()
        .fallback(crate::mesh::mesh_proxy_handler)
        .with_state(state);

    let mgmt_listener = TcpListener::bind(mgmt_addr).await?;
    let mesh_listener = TcpListener::bind(mesh_addr).await?;

    let mgmt_server = axum::serve(mgmt_listener, app);
    let mesh_server = axum::serve(mesh_listener, mesh_app);

    tokio::try_join!(
        async { mgmt_server.await.map_err(anyhow::Error::from) },
        async { mesh_server.await.map_err(anyhow::Error::from) }
    )?;

    Ok(())
}

async fn stats_handler() -> &'static str {
    "OK"
}
