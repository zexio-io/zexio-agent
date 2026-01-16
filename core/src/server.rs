use crate::{
    config::Settings, deploy, middleware, monitor, project, services, state::AppState, streams,
};
use axum::middleware as axum_middleware;
use axum::{
    routing::{delete, get, post},
    Router,
};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

pub async fn start(settings: Settings) -> anyhow::Result<()> {
    // Application state
    info!("📦 Initializing application state...");
    let state = AppState::new(settings.clone())?;
    info!("✅ Application state ready");

    // Protected routes (require authentication in cloud mode, open in standalone)
    let protected_routes = Router::new()
        .route(
            "/projects",
            post(project::create_project).get(project::list_projects_handler),
        )
        .route("/projects/:id", delete(project::delete_project_handler))
        .route("/projects/:id/env", post(project::update_env_handler))
        .route(
            "/projects/:id/domains",
            post(project::add_domain_handler).delete(project::remove_domain_handler),
        )
        .route("/projects/:id/files", get(project::list_files_handler))
        .route("/projects/:id/stats", get(monitor::project_monitor_handler))
        .route(
            "/projects/:id/stats/stream",
            get(monitor::project_monitor_stream),
        ) // SSE!
        .route("/projects/:id/logs", get(streams::project_logs_handler)) // JSON (one-time)
        .route(
            "/projects/:id/logs/stream",
            get(streams::project_logs_stream),
        ) // SSE!
        .route("/projects/:id/deploy", post(deploy::project_deploy_handler))
        .route(
            "/projects/:id/webhook",
            post(deploy::project_deploy_handler),
        )
        .route("/services/install", post(services::install_service_handler))
        .route(
            "/services/uninstall",
            post(services::uninstall_service_handler),
        )
        .route(
            "/firewall/configure",
            post(monitor::configure_firewall_handler),
        )
        .route("/sync", post(monitor::sync_handler))
        .layer(axum_middleware::from_fn_with_state(
            state.clone(),
            middleware::smart_auth_middleware, // Changed from worker_auth_middleware
        ));

    // Public routes (no auth required - for standalone mode and GUI)
    let public_routes = Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/stats", get(monitor::global_stats_handler))
        .route("/stats/stream", get(monitor::global_stats_stream)) // SSE!
        .route("/system/logs", get(streams::worker_logs_handler)) // JSON (one-time)
        .route("/system/logs/stream", get(streams::worker_logs_stream)); // SSE!

    // CORS configuration for GUI access
    let cors = CorsLayer::new()
        .allow_origin(Any) // Allow all origins (Tauri apps)
        .allow_methods(Any) // Allow all HTTP methods
        .allow_headers(Any); // Allow all headers

    // Combine routes with CORS
    let app = public_routes
        .merge(protected_routes)
        .layer(cors) // Add CORS layer
        .with_state(state.clone());

    let mgmt_addr: SocketAddr =
        format!("{}:{}", settings.server.host, settings.server.port).parse()?;

    info!("🌐 Management API listening on http://{}", mgmt_addr);
    info!(
        "🔀 Service Mesh Proxy (Pingora) on port {}",
        settings.server.mesh_port
    );
    info!("");
    info!("📡 Available endpoints:");
    info!("   GET  /health              - Health check");
    info!("   GET  /stats               - System statistics");
    info!("   GET  /stats/stream        - Real-time stats (SSE)");
    info!("");
    info!("✨ Zexio Agent is ready!");

    let mgmt_listener = TcpListener::bind(mgmt_addr).await?;
    let mgmt_server = axum::serve(mgmt_listener, app);

    // 1. Spawn Axum (Management API)
    tokio::spawn(async move {
        if let Err(e) = mgmt_server.await {
            tracing::error!("Management API failed: {}", e);
        }
    });

    // 2. Start Zexio Tunnel Client (Native gRPC)
    let settings_tunnel = settings.clone();
    tokio::spawn(async move {
        // We need the worker_id from identity file
        let identity_path = &settings_tunnel.secrets.identity_path;
        if std::path::Path::new(identity_path).exists() {
            if let Ok(identity_json) = std::fs::read_to_string(identity_path) {
                if let Ok(identity) = serde_json::from_str::<serde_json::Value>(&identity_json) {
                    if let Some(worker_id) = identity["worker_id"].as_str() {
                        use crate::mesh::tunnel::start_tunnel_client;
                        if let Err(e) = start_tunnel_client(settings_tunnel, worker_id.to_string()).await {
                            tracing::error!("Zexio Tunnel failed: {}", e);
                        }
                    }
                }
            }
        }
    });

    // 3. Run Pingora (Mesh Proxy)
    // Pingora manages its own runtime/threads, so we run it in a blocking task
    // to avoid blocking the Tokio executor of the main thread (although main is effectively waiting here).
    let mesh_port = settings.server.mesh_port;
    let state_clone = state.clone();

    tokio::task::spawn_blocking(move || {
        use crate::mesh::zexio_mesh::ZexioMeshLogic;
        use pingora::proxy::http_proxy_service;
        use pingora::server::Server;
        use pingora::services::Service;

        let mut server = Server::new(None).expect("Failed to initialize Pingora server");
        server.bootstrap();

        let mut proxy =
            http_proxy_service(&server.configuration, ZexioMeshLogic { state: state_clone });
        proxy.add_tcp(&format!("0.0.0.0:{}", mesh_port));

        server.add_service(proxy);
        server.run_forever();
    })
    .await?;

    Ok(())
}
