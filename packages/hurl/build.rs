use cc::Build;
use std::path::Path;

fn main() {
    let project_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let native_src = project_root.join("native");
    Build::new()
        .file(native_src.join("libxml.c"))
        .flag_if_supported("-Wno-unused-parameter") // unused parameter in silent callback
        .compile("mylib");
}
