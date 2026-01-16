pub mod firewall;
pub mod proxy;
pub mod tunnel;
pub mod zexio_mesh;

pub mod node_sync {
    tonic::include_proto!("zexio.node.v1");
}
