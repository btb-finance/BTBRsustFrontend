use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=src/*");
    println!("cargo:rerun-if-changed=index.html");

    // Create dist directory if it doesn't exist
    let dist_path = Path::new("dist");
    if !dist_path.exists() {
        std::fs::create_dir_all(dist_path).expect("Failed to create dist directory");
    }

    // Copy index.html to dist
    std::fs::copy("index.html", dist_path.join("index.html"))
        .expect("Failed to copy index.html to dist");

    // Run wasm-bindgen
    let status = Command::new("wasm-bindgen")
        .args(&[
            "target/wasm32-unknown-unknown/release/trunk-template.wasm",
            "--out-dir", "dist",
            "--target", "web",
        ])
        .status()
        .expect("Failed to execute wasm-bindgen");

    if !status.success() {
        panic!("wasm-bindgen failed");
    }
}
