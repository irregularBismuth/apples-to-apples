fn main() -> anyhow::Result<()> {
    let mut config = prost_build::Config::new();
    config.include_file("mod.rs");

    config.compile_protos(
        &[
            "proto/common.proto",
            "proto/client.proto",
            "proto/server.proto",
        ],
        &["proto"],
    )?;

    println!("cargo:rerun-if-changed=proto/common.proto");
    println!("cargo:rerun-if-changed=proto/client.proto");
    println!("cargo:rerun-if-changed=proto/server.proto");
    Ok(())
}
