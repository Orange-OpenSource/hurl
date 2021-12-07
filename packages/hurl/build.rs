use cc::Build;
use std::path::Path;

#[cfg(windows)]
use winres::WindowsResource;

#[cfg(windows)]
fn set_icon() {
    let mut res = WindowsResource::new();
    res.set_icon("../../ci/windows/logo.ico");
    res.compile().unwrap();
}

#[cfg(unix)]
fn set_icon() {}

fn main() {
    let project_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let native_src = project_root.join("native");
    set_icon();
    Build::new()
        .file(native_src.join("libxml.c"))
        .flag_if_supported("-Wno-unused-parameter") // unused parameter in silent callback
        .compile("mylib");
}
