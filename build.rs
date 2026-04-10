fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_files = &[
        "proto/types.proto",
        "proto/build.proto",
        "proto/clone_ws.proto",
        "proto/data_stream.proto",
        "proto/execution.proto",
        "proto/introspection.proto",
        "proto/security.proto",
        "proto/system_utils.proto",
        "proto/file_system.proto",
        "proto/settings.proto",
        "proto/terminal.proto",
        "proto/workspace.proto",
    ];

    let proto_dirs = &["proto"];

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile(proto_files, proto_dirs)?;

    Ok(())
}
