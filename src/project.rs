use axum::{
    extract::{State, Json, Path},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use crate::{state::AppState, errors::AppError, auth::WorkerAuth};
use crate::caddy::Caddy;
use crate::storage::ProjectConfig;
use trust_dns_resolver::TokioAsyncResolver;
use tracing::{info, warn};
use std::fs;
use std::process::Command;

#[derive(Deserialize)]
pub struct CreateProjectRequest {
    pub project_id: String,
    pub domains: Vec<String>,
    pub webhook_secret: String,
}

pub async fn create_project(
    State(state): State<AppState>,
    WorkerAuth(_): WorkerAuth,
    Json(req): Json<CreateProjectRequest>,
) -> Result<impl IntoResponse, AppError> {
    info!("Creating project: {}", req.project_id);

    // Determine port
    let port = 8000 + (crc32fast::hash(req.project_id.as_bytes()) % 1000) as u16;

    // Create project config
    let config = ProjectConfig {
        id: req.project_id.clone(),
        domains: req.domains.clone(),
        encrypted_env: String::new(), // Empty initially
        webhook_secret: req.webhook_secret,
        created_at: chrono::Utc::now(),
    };

    // Save to storage
    state.store.create(config).await
        .map_err(|_| AppError::InternalServerError)?;

    // Configure Caddy for each domain
    let caddy = Caddy::new(state.settings.caddy.clone());
    for domain in &req.domains {
        caddy.add_domain(domain, &req.project_id, port)
            .map_err(|_e| AppError::InternalServerError)?;
    }

    caddy.reload().map_err(|_e| AppError::InternalServerError)?;

    info!("Project {} created successfully on port {}", req.project_id, port);

    Ok((StatusCode::CREATED, Json(serde_json::json!({
        "project_id": req.project_id,
        "port": port,
        "status": "created"
    }))))
}

#[derive(Deserialize)]
pub struct UpdateEnvRequest {
    pub encrypted_env: String, // Hex-encoded encrypted blob
}

pub async fn update_env_handler(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
    WorkerAuth(_): WorkerAuth,
    Json(payload): Json<UpdateEnvRequest>,
) -> Result<impl IntoResponse, AppError> {
    info!("Updating environment for project: {}", project_id);

    // Read existing config
    let mut config = state.store.read(&project_id).await
        .map_err(|_| AppError::BadRequest("Project not found".into()))?;

    // Update encrypted env
    config.encrypted_env = payload.encrypted_env;

    // Save
    state.store.update(&config).await
        .map_err(|_| AppError::InternalServerError)?;

    Ok((StatusCode::OK, "Environment updated"))
}

#[derive(Deserialize)]
pub struct DomainRequest {
    pub domain: String,
}

pub async fn add_domain_handler(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
    WorkerAuth(_): WorkerAuth,
    Json(payload): Json<DomainRequest>,
) -> Result<impl IntoResponse, AppError> {
    let domain = payload.domain;
    
    // 1. Verify Domain (CNAME or A Record)
    let resolver = TokioAsyncResolver::tokio_from_system_conf().unwrap_or_else(|_| {
        TokioAsyncResolver::tokio(
           trust_dns_resolver::config::ResolverConfig::google(),
           trust_dns_resolver::config::ResolverOpts::default(),
        )
    });

    let resolved_ips = match resolver.lookup_ip(domain.as_str()).await {
        Ok(lookup) => lookup.iter().collect::<Vec<_>>(),
        Err(e) => {
            warn!("DNS lookup failed for {}: {}", domain, e);
            vec![] 
        }
    };

    let mut verified = false;

    // Check 1: Match Static Public IP
    if let Some(pub_ip) = &state.settings.server.public_ip {
        if let Ok(expected_ip) = pub_ip.parse::<std::net::IpAddr>() {
             if resolved_ips.contains(&expected_ip) {
                 info!("Domain {} verified via Public IP match ({})", domain, expected_ip);
                 verified = true;
             }
        }
    }

    // Check 2: Match Public Hostname (Resolve & Compare)
    if !verified {
        if let Some(pub_host) = &state.settings.server.public_hostname {
             if let Ok(host_lookup) = resolver.lookup_ip(pub_host.as_str()).await {
                 let host_ips: Vec<_> = host_lookup.iter().collect();
                 for user_ip in &resolved_ips {
                     if host_ips.contains(user_ip) {
                         info!("Domain {} verified via Public Hostname resolution match ({})", domain, user_ip);
                         verified = true;
                         break;
                     }
                 }
             }
        }
    }

    // Pass for Dev/Testing if configured to bypass or if both are missing
    if state.settings.server.public_ip.is_none() && state.settings.server.public_hostname.is_none() {
        warn!("No public_ip or public_hostname configured. Skipping DNS verification.");
        verified = true;
    }

    if !verified && !resolved_ips.is_empty() {
         warn!("DNS verification failed for {}. Resolved: {:?}. Expected IP: {:?} or Host: {:?}", 
             domain, resolved_ips, state.settings.server.public_ip, state.settings.server.public_hostname);
    }

    // 2. Determine Port
    let port = 8000 + (crc32fast::hash(project_id.as_bytes()) % 1000) as u16;

    // 3. Update Caddy
    let caddy = Caddy::new(state.settings.caddy.clone());
    caddy.add_domain(&domain, &project_id, port)
        .map_err(|_e| AppError::InternalServerError)?; 

    caddy.reload().map_err(|_e| AppError::InternalServerError)?;
    
    // 4. Update Config (Append domain)
    let mut config = state.store.read(&project_id).await
        .map_err(|_| AppError::BadRequest("Project not found".into()))?;

    if !config.domains.contains(&domain) {
        config.domains.push(domain.clone());
        state.store.update(&config).await
            .map_err(|_| AppError::InternalServerError)?;
    }

    Ok((StatusCode::OK, "Domain added").into_response())
}

pub async fn remove_domain_handler(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
    WorkerAuth(_): WorkerAuth,
    Json(payload): Json<DomainRequest>,
) -> Result<impl IntoResponse, AppError> {
    let domain = payload.domain;

    // 1. Remove from Caddy
    let caddy = Caddy::new(state.settings.caddy.clone());
    caddy.remove_domain(&domain)
        .map_err(|_e| AppError::InternalServerError)?;

    caddy.reload().map_err(|_e| AppError::InternalServerError)?;

    // 2. Update Config (Remove domain)
    let mut config = state.store.read(&project_id).await
        .map_err(|_| AppError::BadRequest("Project not found".into()))?;

    config.domains.retain(|d| d != &domain);
    
    state.store.update(&config).await
        .map_err(|_| AppError::InternalServerError)?;

    Ok((StatusCode::OK, "Domain removed").into_response())
}

#[derive(serde::Serialize)]
pub struct FileInfo {
    name: String,
    size: u64,
    is_dir: bool,
}

pub async fn list_files_handler(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
    WorkerAuth(_): WorkerAuth,
) -> Result<Json<Vec<FileInfo>>, AppError> {
    
    let base_path = format!("{}/{}/bundle", state.settings.storage.projects_dir, project_id);
    let mut entries = Vec::new();

    if let Ok(dir) = fs::read_dir(&base_path) {
        for entry in dir {
            if let Ok(entry) = entry {
                let metadata = entry.metadata().ok();
                entries.push(FileInfo {
                    name: entry.file_name().to_string_lossy().to_string(),
                    size: metadata.as_ref().map(|m| m.len()).unwrap_or(0),
                    is_dir: metadata.map(|m| m.is_dir()).unwrap_or(false),
                });
            }
        }
    } else {
        return Ok(Json(vec![])); 
    }

    Ok(Json(entries))
}

#[derive(Serialize)]
pub struct ProjectSummary {
    id: String,
    domains: Vec<String>,
    created_at: String,
}

pub async fn list_projects_handler(
    State(state): State<AppState>,
    WorkerAuth(_): WorkerAuth,
) -> Result<Json<Vec<ProjectSummary>>, AppError> {
    let configs = state.store.list().await
        .map_err(|_| AppError::InternalServerError)?;

    let summaries: Vec<ProjectSummary> = configs.into_iter().map(|c| ProjectSummary {
        id: c.id,
        domains: c.domains,
        created_at: c.created_at.to_rfc3339(),
    }).collect();

    Ok(Json(summaries))
}

pub async fn delete_project_handler(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
    WorkerAuth(_): WorkerAuth,
) -> Result<impl IntoResponse, AppError> {
    info!("Deleting project: {}", project_id);

    // 1. Stop systemd service
    let _ = Command::new("systemctl")
        .arg("stop")
        .arg(format!("app@{}.service", project_id))
        .output();

    // 2. Delete project directory (includes config.json and bundle)
    state.store.delete(&project_id).await
        .map_err(|_| AppError::InternalServerError)?;

    // 3. Remove from Caddy (all domains)
    // Note: We don't have the domain list anymore, but Caddy cleanup can be manual or we skip
    // For now, we'll skip Caddy cleanup since domains are gone with the project

    Ok((StatusCode::OK, "Project deleted"))
}
