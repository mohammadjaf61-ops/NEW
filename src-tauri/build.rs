fn main() {
    // Skip resource compilation to avoid icon errors
    tauri_build::try_build(tauri_build::Attributes::new()).expect("failed to run build script");
}
