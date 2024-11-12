/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2024 Orange
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
use hurl_core::typing::{BytesPerSec, Count};
use std::time::Duration;

use crate::http::request::RequestedHttpVersion;
use crate::http::IpResolve;

#[derive(Debug, Clone)]
pub struct ClientOptions {
    pub aws_sigv4: Option<String>,
    pub cacert_file: Option<String>,
    pub client_cert_file: Option<String>,
    pub client_key_file: Option<String>,
    pub compressed: bool,
    pub connect_timeout: Duration,
    pub connects_to: Vec<String>,
    pub cookie_input_file: Option<String>,
    pub follow_location: bool,
    pub follow_location_trusted: bool,
    pub fresh_connect: bool,
    pub http_version: RequestedHttpVersion,
    pub insecure: bool,
    pub ip_resolve: IpResolve,
    pub max_filesize: Option<u64>,
    pub max_recv_speed: Option<BytesPerSec>,
    pub max_redirect: Count,
    pub max_send_speed: Option<BytesPerSec>,
    pub netrc: bool,
    pub netrc_file: Option<String>,
    pub netrc_optional: bool,
    pub no_proxy: Option<String>,
    pub path_as_is: bool,
    pub proxy: Option<String>,
    pub resolves: Vec<String>,
    pub ssl_no_revoke: bool,
    pub timeout: Duration,
    pub unix_socket: Option<String>,
    pub user: Option<String>,
    pub user_agent: Option<String>,
    pub verbosity: Option<Verbosity>,
}

// FIXME/ we could implement copy here
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Verbosity {
    Verbose,
    VeryVerbose,
}

impl Default for ClientOptions {
    fn default() -> Self {
        ClientOptions {
            aws_sigv4: None,
            cacert_file: None,
            client_cert_file: None,
            client_key_file: None,
            compressed: false,
            connect_timeout: Duration::from_secs(300),
            connects_to: vec![],
            cookie_input_file: None,
            follow_location: false,
            follow_location_trusted: false,
            fresh_connect: false,
            http_version: RequestedHttpVersion::default(),
            insecure: false,
            ip_resolve: IpResolve::default(),
            max_filesize: None,
            max_recv_speed: None,
            max_redirect: Count::Finite(50),
            max_send_speed: None,
            netrc: false,
            netrc_file: None,
            netrc_optional: false,
            no_proxy: None,
            path_as_is: false,
            proxy: None,
            resolves: vec![],
            ssl_no_revoke: false,
            timeout: Duration::from_secs(300),
            unix_socket: None,
            user: None,
            user_agent: None,
            verbosity: None,
        }
    }
}
