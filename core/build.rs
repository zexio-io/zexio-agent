fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "linux")]
    std::env::set_var("PROTOC", protobuf_src::protoc());

    tonic_build::compile_protos("proto/node_sync.proto")?;
    Ok(())
}
