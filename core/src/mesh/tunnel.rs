use crate::config::Settings;
use crate::mesh::node_sync::{
    node_sync_service_client::NodeSyncServiceClient, NodeConnectionRequest, NodeStatsRequest,
    ServiceStatus, TunnelPacket,
};
use crate::storage::ProjectStore; // Added import
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use sysinfo::System;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;

use tonic::Request;
use tracing::{debug, error, info, warn};

pub async fn start_tunnel_client(settings: Settings, node_id: String, target_port: u16) -> anyhow::Result<()> {
    // 1. Determine Relay URL
    let relay_url =
        std::env::var("RELAY_URL").unwrap_or_else(|_| "http://127.0.0.1:50051".to_string());
    
    // Retry Loop
    loop {
        info!(
            "🔗 Connecting to Zexio Relay at {} (Node ID: {})...",
            relay_url, node_id
        );

        // 2. Connect
        let client_res = NodeSyncServiceClient::connect(relay_url.clone()).await;
        
        match client_res {
            Ok(mut client) => {
                // 3. Handshake (Authentication)
                let auth_token = settings
                    .secrets
                    .worker_secret
                    .clone()
                    .unwrap_or_else(|| "dev-token".to_string());

                let os_info = System::long_os_version().unwrap_or("Unknown OS".into());

                let auth_req = Request::new(NodeConnectionRequest {
                    node_id: node_id.clone(),
                    auth_token,
                    os_type: os_info,
                    version: env!("CARGO_PKG_VERSION").to_string(),
                });

                match client.on_connect(auth_req).await {
                    Ok(auth_res) => {
                        if !auth_res.get_ref().success {
                            error!("❌ Authentication Failed: {}", auth_res.get_ref().message);
                            // If auth fails, waiting a bit longer might be wise, or just exit if we want strictly correct auth
                            // For now, retry in 10s
                            tokio::time::sleep(Duration::from_secs(10)).await;
                            continue; 
                        }
                        info!("✅ Authenticated! Tunnel Active.");
                        
                        // 4. Start Heartbeat Loop (Health Check)
                        let mut stats_client = client.clone();
                        let node_id_stats = node_id.clone();
                        let settings_for_stats = settings.clone();

                        // We abort this task if we disconnect, to avoid zombie stats
                        let heartbeat_handle = tokio::spawn(async move {
                            let project_store = ProjectStore::new(settings_for_stats.storage.projects_dir);
                            let mut sys = System::new_all();

                            let stats_stream = async_stream::stream! {
                                loop {
                                    sys.refresh_all(); // Refresh system stats

                                    // 1. Gather Service Statuses
                                    let mut service_statuses = Vec::new();
                                    if let Ok(projects) = project_store.list().await {
                                        for p in projects {
                                            let port = 8000 + (crc32fast::hash(p.id.as_bytes()) % 1000) as u16;
                                            let target = format!("127.0.0.1:{}", port);

                                            let start = std::time::Instant::now();
                                            let (status, latency) = match TcpStream::connect(&target).await {
                                                Ok(_) => ("UP".to_string(), start.elapsed().as_millis() as u32),
                                                Err(_) => ("DOWN".to_string(), 0),
                                            };

                                            service_statuses.push(ServiceStatus {
                                                id: p.id,
                                                name: p.domains.first().cloned().unwrap_or_else(|| "unknown".to_string()),
                                                status,
                                                latency_ms: latency,
                                                domains: p.domains.clone(),
                                            });
                                        }
                                    }

                                    // Calculate Resource Usage
                                    let cpu_usage = sys.global_cpu_info().cpu_usage();
                                    let memory_usage = (sys.used_memory() as f32 / sys.total_memory() as f32) * 100.0;

                                    // 2. Yield Stats
                                    yield NodeStatsRequest {
                                        node_id: node_id_stats.clone(),
                                        cpu_usage,
                                        memory_usage,
                                        disk_usage: 0.0,
                                        timestamp: Some(std::time::SystemTime::now().into()),
                                        services: service_statuses,
                                    };
                                    tokio::time::sleep(Duration::from_secs(5)).await;
                                }
                            };
                            if let Err(e) = stats_client.sync_stats(Request::new(stats_stream)).await {
                                warn!("Heartbeat stream finished/failed: {}", e);
                            }
                        });


                        // 5. Setup Tunnel Channels
                        let (tx, rx) = mpsc::channel(32);
                        let outbound_stream = ReceiverStream::new(rx);

                        // Send Initial Packet
                        let init_packet = TunnelPacket {
                            node_id: node_id.clone(),
                            request_id: "".to_string(),
                            data: vec![],
                            is_init: true,
                            is_eof: false,
                        };
                        
                        if let Err(e) = tx.send(init_packet).await {
                             error!("Failed to send init packet, reconnecting... Error: {}", e);
                             heartbeat_handle.abort();
                             tokio::time::sleep(Duration::from_secs(5)).await;
                             continue;
                        }

                        // Open Bi-directional Stream
                        match client.open_tunnel(Request::new(outbound_stream)).await {
                            Ok(response) => {
                                let mut inbound = response.into_inner();
                                info!("🚀 Tunnel Stream Established. Forwarding to local port...");

                                let active_sessions: Arc<Mutex<HashMap<String, mpsc::Sender<Vec<u8>>>>> = Arc::new(Mutex::new(HashMap::new()));
                                info!("🎯 Tunnel Target: 127.0.0.1:{}", target_port);

                                // --- Tunnel Loop ---
                                while let Some(result) = inbound.next().await {
                                    match result {
                                        Ok(pkt) => {
                                            // Process Packet Code (same as before)
                                            let request_id = pkt.request_id.clone();
                                            let mut sessions = active_sessions.lock().await;

                                            if pkt.is_eof {
                                                sessions.remove(&request_id);
                                                continue;
                                            }

                                            if !sessions.contains_key(&request_id) {
                                                // New Session
                                                let (session_tx, mut session_rx) = mpsc::channel::<Vec<u8>>(64);
                                                sessions.insert(request_id.clone(), session_tx);

                                                let node_id_inner = node_id.clone();
                                                let request_id_inner = request_id.clone();
                                                let tx_to_relay = tx.clone();
                                                let target = format!("127.0.0.1:{}", target_port);

                                                tokio::spawn(async move {
                                                    // (Same Local Proxy Logic)
                                                    debug!("Tunnel [{}] -> Connecting to {}", request_id_inner, target);
                                                    match TcpStream::connect(&target).await {
                                                        Ok(mut stream) => {
                                                            if !pkt.data.is_empty() {
                                                                if let Err(e) = stream.write_all(&pkt.data).await {
                                                                    error!("Failed to write to local app: {}", e);
                                                                    return;
                                                                }
                                                            }
                                                            let (mut reader, mut writer) = stream.into_split();
                                                            let req_id = request_id_inner.clone();
                                                            let n_id = node_id_inner.clone();
                                                            let tx_grpc = tx_to_relay.clone();

                                                            // Reader Task
                                                            tokio::spawn(async move {
                                                                let mut buf = [0u8; 8192];
                                                                loop {
                                                                    match reader.read(&mut buf).await {
                                                                        Ok(0) => {
                                                                            let _ = tx_grpc.send(TunnelPacket {
                                                                                node_id: n_id, request_id: req_id, data: vec![], is_init: false, is_eof: true,
                                                                            }).await;
                                                                            break;
                                                                        }
                                                                        Ok(n) => {
                                                                            let _ = tx_grpc.send(TunnelPacket {
                                                                                node_id: n_id.clone(), request_id: req_id.clone(), data: buf[..n].to_vec(), is_init: false, is_eof: false,
                                                                            }).await;
                                                                        }
                                                                        Err(_) => break,
                                                                    }
                                                                }
                                                            });

                                                            // Writer Loop
                                                            while let Some(data) = session_rx.recv().await {
                                                                if (writer.write_all(&data).await).is_err() { break; }
                                                            }
                                                        }
                                                        Err(e) => {
                                                            warn!("Failed to connect to local app at {}: {}", target, e);
                                                        }
                                                    }
                                                });
                                            } else {
                                                if let Some(s_tx) = sessions.get(&request_id) {
                                                    let _ = s_tx.send(pkt.data).await;
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            error!("❌ Tunnel Stream Error: {}. Reconnecting...", e);
                                            break; 
                                        }
                                    }
                                }
                                info!("⚠️  Tunnel stream closed. Cleaning up...");
                                heartbeat_handle.abort(); // Stop heartbeat
                            }
                            Err(e) => {
                                error!("❌ Failed to open tunnel stream: {}. Retrying...", e);
                                heartbeat_handle.abort();
                            }
                        }
                    }
                    Err(e) => {
                          error!("❌ Failed to call on_connect: {}. Retrying...", e);
                    }
                }
            }
            Err(e) => {
                error!("❌ Failed to connect to relay: {}. Retrying in 5s...", e);
            }
        }
        
        // Wait before reconnecting to avoid spam
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
