/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2025 Orange
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

use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CurlVersionInfo {
    pub host: String,
    pub libraries: Vec<String>,
    pub features: Vec<String>,
}

/// Returns the libraries and features of libcurl.
///
/// Output should be similar to `curl --version`
/// - <https://github.com/curl/curl/blob/master/lib/version.c>
/// - <https://github.com/curl/curl/blob/master/src/tool_help.c>
pub fn libcurl_version_info() -> CurlVersionInfo {
    let version = curl::Version::get();
    let host = version.host().to_string();
    let mut libraries = vec![format!("libcurl/{}", version.version())];
    if let Some(s) = version.ssl_version() {
        libraries.push(s.to_string());
    }
    if let Some(s) = version.libz_version() {
        libraries.push(format!("zlib/{s}"));
    }
    if let Some(s) = version.brotli_version() {
        libraries.push(format!("brotli/{s}"));
    }
    if let Some(s) = version.zstd_version() {
        libraries.push(format!("zstd/{s}"));
    }
    if let Some(s) = version.ares_version() {
        libraries.push(format!("c-ares/{s}"));
    }
    if let Some(s) = version.libidn_version() {
        libraries.push(format!("libidn2/{s}"));
    }
    if let Some(s) = version.iconv_version_num() {
        if s != 0 {
            libraries.push(format!("iconv/{s}"));
        }
    }
    if let Some(s) = version.libssh_version() {
        libraries.push(s.to_string());
    }
    if let Some(s) = version.nghttp2_version() {
        libraries.push(format!("nghttp2/{s}"));
    }
    if let Some(s) = version.quic_version() {
        libraries.push(format!("quic/{s}"));
    }
    if let Some(s) = version.hyper_version() {
        libraries.push(format!("hyper/{s}"));
    }
    if let Some(s) = version.gsasl_version() {
        libraries.push(format!("libgsal/{s}"));
    }

    // FIXME: some flags are not present in crates curl-rust.
    // See https://github.com/alexcrichton/curl-rust/issues/464
    // See https://github.com/curl/curl/blob/master/include/curl/curl.h for all curl flags
    // See https://github.com/alexcrichton/curl-rust/blob/main/curl-sys/lib.rs for curl-rust flags
    // Not defined in curl-rust:
    // - CURL_VERSION_GSSAPI        (1<<17)
    // - CURL_VERSION_KERBEROS5     (1<<18)
    // - CURL_VERSION_PSL           (1<<20)
    // - CURL_VERSION_HTTPS_PROXY   (1<<21)
    // - CURL_VERSION_MULTI_SSL     (1<<22)
    // - CURL_VERSION_THREADSAFE    (1<<30)

    let all_features = HashMap::from([
        ("AsynchDNS", version.feature_async_dns()),
        ("Debug", version.feature_debug()),
        ("IDN", version.feature_idn()),
        ("IPv6", version.feature_ipv6()),
        ("Largefile", version.feature_largefile()),
        ("Unicode", version.feature_unicode()),
        ("SSPI", version.feature_sspi()),
        ("SPNEGO", version.feature_spnego()),
        ("NTLM", version.feature_ntlm()),
        ("NTLM_WB", version.feature_ntlm_wb()),
        ("SSL", version.feature_ssl()),
        ("libz", version.feature_libz()),
        ("brotli", version.feature_brotli()),
        ("zstd", version.feature_zstd()),
        ("CharConv", version.feature_conv()),
        ("TLS-SRP", version.feature_tlsauth_srp()),
        ("HTTP2", version.feature_http2()),
        ("HTTP3", version.feature_http3()),
        ("UnixSockets", version.feature_unix_domain_socket()),
        ("alt-svc", version.feature_altsvc()),
        ("HSTS", version.feature_hsts()),
        ("gsasl", version.feature_gsasl()),
        ("GSS-Negotiate", version.feature_gss_negotiate()),
    ]);
    let mut features: Vec<String> = vec![];
    for (k, v) in all_features.iter() {
        if *v {
            features.push(k.to_string());
        }
    }
    features.sort_by_key(|k| k.to_lowercase());

    CurlVersionInfo {
        host,
        libraries,
        features,
    }
}
