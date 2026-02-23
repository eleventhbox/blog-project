fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:warning=This is a server build script warning");
    println!("cargo:rerun-if-changed=proto/blog.proto");
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile(&["proto/blog.proto"], &["proto"])?;
    Ok(())
}
