fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out = std::env::var("OUT_DIR").unwrap();
    println!("out: {}", out);
    println!("hello, build.rs");

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir("src")
        .compile(&["raft_service.proto"], &["proto/"])?;
    Ok(())
}
