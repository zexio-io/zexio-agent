use async_trait::async_trait;
use pingora::prelude::*;
use std::sync::Arc;
use crate::state::AppState;
use tracing::{error, debug, info};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use redis::AsyncCommands;

pub struct ZexioMeshLogic {
    pub state: AppState,
}

#[derive(Debug, Serialize, Deserialize)]
struct ServiceTokenClaims {
    #[serde(rename = "userId")]
    user_id: String,
    #[serde(rename = "orgId")]
    org_id: String,
    #[serde(rename = "sourceService")]
    source_service: String,
    #[serde(rename = "targetService")]
    target_service: String,
    #[serde(rename = "workerId")]
    worker_id: Option<String>,
}

pub struct MeshContext {
    pub target_host: String,
    pub target_port: u16,
    pub target_org_id: String,
}

#[async_trait]
impl ProxyHttp for ZexioMeshLogic {
    type CTX = Option<MeshContext>;

    fn new_ctx(&self) -> Self::CTX {
        None
    }

    async fn request_filter(&self, session: &mut Session, ctx: &mut Self::CTX) -> Result<bool> {
        let host_header = session.req_header().headers.get("Host").map(|v| v.to_str().unwrap_or("")).unwrap_or("");
        let host = host_header.split(':').next().unwrap_or(host_header).to_string();

        debug!("ZexioMesh: Receiving request for host: {}", host);

        // 1. Resolve Host
        let (target_host, target_port, target_org_id) = match self.resolve_mesh_dns(&host).await {
            Ok(res) => res,
            Err(_) => {
                let _ = session.respond_error(404).await;
                return Ok(true); // Handled
            }
        };

        // 2. Auth Validation
        let auth_header = session.req_header().headers.get("Authorization").map(|v| v.to_str().unwrap_or(""));
        
        let is_authorized = if let Some(auth_str) = auth_header {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                let mut validation = Validation::new(Algorithm::HS256);
                validation.set_issuer(&["zexio-service-mesh"]);
                
                let decoding_key = DecodingKey::from_secret(self.state.mesh_jwt_secret.as_bytes());
                
                match decode::<ServiceTokenClaims>(token, &decoding_key, &validation) {
                    Ok(token_data) => {
                         token_data.claims.org_id == target_org_id
                    },
                    Err(e) => {
                        error!("Mesh token validation failed: {}", e);
                        false
                    }
                }
            } else {
                false
            }
        } else {
            false
        };

        if !is_authorized {
            error!("Tenant Violation: Unauthorized access to {}", host);
            let _ = session.respond_error(403).await;
            return Ok(true);
        }

        // Store in context for upstream_peer
        *ctx = Some(MeshContext {
            target_host,
            target_port,
            target_org_id,
        });

        Ok(false) // Continue to upstream_peer
    }

    async fn upstream_peer(&self, _session: &mut Session, ctx: &mut Self::CTX) -> Result<Box<HttpPeer>> {
        let ctx = ctx.as_ref().ok_or_else(|| pingora::Error::new(ErrorType::InternalError))?;

        info!("Proxying to {}:{}", ctx.target_host, ctx.target_port);

        let peer = Box::new(HttpPeer::new(
            (ctx.target_host.as_str(), ctx.target_port),
            false, // TLS? For now false (internal mesh)
            "".to_string(), // SNI
        ));
        
        Ok(peer)
    }
}

impl ZexioMeshLogic {
    // Helper function (copied logic from proxy.rs but adapted)
    async fn resolve_mesh_dns(&self, host: &str) -> Result<(String, u16, String), ()> {
         // Format: {userId}.{serviceSlug}.zexio.internal
         if host.ends_with(".zexio.internal") {
            let parts: Vec<&str> = host.split('.').collect();
            
            if parts.len() >= 4 {
                let mut conn = self.state.redis.get_async_connection().await.map_err(|_| ())?;
                let redis_key = format!("service:{}", host);
                let service_info: std::collections::HashMap<String, String> = conn.hgetall(redis_key).await.map_err(|_| ())?;
    
                if let Some(target_ip) = service_info.get("worker_ip") {
                    let port = service_info.get("port")
                        .and_then(|p| p.parse().ok())
                        .unwrap_or(80);
                    
                    let owner_id = service_info.get("owner_id").cloned().unwrap_or_default();
                    
                    let is_on_this_worker = match &self.state.settings.server.public_ip {
                        Some(ip) => target_ip == ip,
                        None => target_ip == "127.0.0.1" || target_ip == "localhost",
                    };
    
                    if is_on_this_worker {
                         return Ok(("127.0.0.1".to_string(), port, owner_id));
                    }
                    return Ok((target_ip.clone(), port, owner_id));
                }
            }
    
            // Legacy
            let project_id = host.replace(".zexio.internal", "");
            let port = 8000 + (crc32fast::hash(project_id.as_bytes()) % 1000) as u16;
            return Ok(("127.0.0.1".to_string(), port, "".to_string()));
         }
    
         if host.ends_with(".zexio.app") {
            let prefix = host.replace(".zexio.app", "");
            let project_id = if let Some(uuid) = prefix.split("--").last() {
                uuid.to_string() 
            } else {
                prefix
            };
            let port = 8000 + (crc32fast::hash(project_id.as_bytes()) % 1000) as u16;
            return Ok(("127.0.0.1".to_string(), port, "".to_string()));
         }
         Err(())
    }
}
