pub mod firewall;
pub mod proxy;
pub mod tunnel;
pub mod zexio_mesh;

#[allow(clippy::unwrap_used)]
pub mod node_sync {
    tonic::include_proto!("zexio.node.v1");
}
