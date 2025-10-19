fn main() {
    // Skip tauri-build completely - no icons needed
    println!("cargo:rustc-cfg=desktop");
}
