/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2022 Orange
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *          http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */

use cc::Build;
use std::path::Path;

#[cfg(windows)]
use winres::WindowsResource;

#[cfg(windows)]
fn set_icon() {
    let mut res = WindowsResource::new();
    res.set_icon("../../bin/windows/logo.ico");
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
