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

///
/// get libcurl version
///
// Output should be similar to curl
// https://github.com/curl/curl/blob/master/lib/version.c
//
// Remarks:
// 1) you must make the data returned from libcurl uniform: prefix, string version
// 2) can not find libpsl info in curl crate
//
pub fn libcurl_version_info() -> Vec<String> {
    let version = curl::Version::get();
    let mut versions = vec![format!("libcurl/{}", version.version())];
    if let Some(s) = version.ssl_version() {
        versions.push(s.to_string());
    }
    if let Some(s) = version.libz_version() {
        versions.push(format!("zlib/{}", s));
    }
    if let Some(s) = version.brotli_version() {
        versions.push(format!("brotli/{}", s));
    }
    if let Some(s) = version.zstd_version() {
        versions.push(format!("zstd/{}", s));
    }
    if let Some(s) = version.ares_version() {
        versions.push(format!("c-ares/{}", s));
    }
    if let Some(s) = version.libidn_version() {
        versions.push(format!("libidn2/{}", s));
    }
    if let Some(s) = version.iconv_version_num() {
        if s != 0 {
            versions.push(format!("iconv/{}", s));
        }
    }
    if let Some(s) = version.libssh_version() {
        versions.push(format!("libssh/{}", s));
    }
    if let Some(s) = version.nghttp2_version() {
        versions.push(format!("nghttp2/{}", s));
    }
    if let Some(s) = version.quic_version() {
        versions.push(format!("quic/{}", s));
    }
    if let Some(s) = version.hyper_version() {
        versions.push(format!("hyper/{}", s));
    }
    if let Some(s) = version.gsasl_version() {
        versions.push(format!("libgsal/{}", s));
    }
    versions
}
