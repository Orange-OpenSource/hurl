/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2023 Orange
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
use std::time::Duration;

use hurl_core::ast::Retry;

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
    pub http_version: RequestedHttpVersion,
    pub insecure: bool,
    pub ip_resolve: IpResolve,
    pub max_redirect: Option<usize>,
    pub netrc: bool,
    pub netrc_file: Option<String>,
    pub netrc_optional: bool,
    pub no_proxy: Option<String>,
    pub path_as_is: bool,
    pub proxy: Option<String>,
    pub resolves: Vec<String>,
    pub retry: Retry,
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
            http_version: RequestedHttpVersion::default(),
            insecure: false,
            ip_resolve: IpResolve::default(),
            max_redirect: Some(50),
            netrc: false,
            netrc_file: None,
            netrc_optional: false,
            no_proxy: None,
            path_as_is: false,
            proxy: None,
            resolves: vec![],
            retry: Retry::None,
            ssl_no_revoke: false,
            timeout: Duration::from_secs(300),
            unix_socket: None,
            user: None,
            user_agent: None,
            verbosity: None,
        }
    }
}

impl ClientOptions {
    /// Returns the list of options for the curl command line equivalent to this [`ClientOptions`].
    pub fn curl_args(&self) -> Vec<String> {
        let mut arguments = vec![];

        if let Some(ref aws_sigv4) = self.aws_sigv4 {
            arguments.push("--aws-sigv4".to_string());
            arguments.push(aws_sigv4.clone());
        }
        if let Some(ref cacert_file) = self.cacert_file {
            arguments.push("--cacert".to_string());
            arguments.push(cacert_file.clone());
        }
        if let Some(ref client_cert_file) = self.client_cert_file {
            arguments.push("--cert".to_string());
            arguments.push(client_cert_file.clone());
        }
        if let Some(ref client_key_file) = self.client_key_file {
            arguments.push("--key".to_string());
            arguments.push(client_key_file.clone());
        }
        if self.compressed {
            arguments.push("--compressed".to_string());
        }
        if self.connect_timeout != ClientOptions::default().connect_timeout {
            arguments.push("--connect-timeout".to_string());
            arguments.push(self.connect_timeout.as_secs().to_string());
        }
        for connect in self.connects_to.iter() {
            arguments.push("--connect-to".to_string());
            arguments.push(connect.clone());
        }
        if let Some(ref cookie_file) = self.cookie_input_file {
            arguments.push("--cookie".to_string());
            arguments.push(cookie_file.clone());
        }
        match self.http_version {
            RequestedHttpVersion::Default => {}
            RequestedHttpVersion::Http10 => arguments.push("--http1.0".to_string()),
            RequestedHttpVersion::Http11 => arguments.push("--http1.1".to_string()),
            RequestedHttpVersion::Http2 => arguments.push("--http2".to_string()),
            RequestedHttpVersion::Http3 => arguments.push("--http3".to_string()),
        }
        if self.insecure {
            arguments.push("--insecure".to_string());
        }
        match self.ip_resolve {
            IpResolve::Default => {}
            IpResolve::IpV4 => arguments.push("--ipv4".to_string()),
            IpResolve::IpV6 => arguments.push("--ipv6".to_string()),
        }
        if self.follow_location_trusted {
            arguments.push("--location-trusted".to_string());
        } else if self.follow_location {
            arguments.push("--location".to_string());
        }
        if self.max_redirect != ClientOptions::default().max_redirect {
            let max_redirect = match self.max_redirect {
                None => -1,
                Some(n) => n as i32,
            };
            arguments.push("--max-redirs".to_string());
            arguments.push(max_redirect.to_string());
        }
        if let Some(filename) = &self.netrc_file {
            arguments.push("--netrc-file".to_string());
            arguments.push(format!("'{filename}'"));
        }
        if self.netrc_optional {
            arguments.push("--netrc-optional".to_string());
        }
        if self.netrc {
            arguments.push("--netrc".to_string());
        }
        if self.path_as_is {
            arguments.push("--path-as-is".to_string());
        }
        if let Some(ref proxy) = self.proxy {
            arguments.push("--proxy".to_string());
            arguments.push(format!("'{proxy}'"));
        }
        for resolve in self.resolves.iter() {
            arguments.push("--resolve".to_string());
            arguments.push(resolve.clone());
        }
        if self.timeout != ClientOptions::default().timeout {
            arguments.push("--timeout".to_string());
            arguments.push(self.timeout.as_secs().to_string());
        }
        if let Some(ref unix_socket) = self.unix_socket {
            arguments.push("--unix-socket".to_string());
            arguments.push(format!("'{unix_socket}'"))
        }
        if let Some(ref user) = self.user {
            arguments.push("--user".to_string());
            arguments.push(format!("'{user}'"));
        }
        if let Some(ref user_agent) = self.user_agent {
            arguments.push("--user-agent".to_string());
            arguments.push(format!("'{user_agent}'"));
        }
        arguments
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curl_args() {
        assert!(ClientOptions::default().curl_args().is_empty());

        assert_eq!(
            ClientOptions {
                aws_sigv4: None,
                cacert_file: None,
                client_cert_file: None,
                client_key_file: None,
                compressed: true,
                connect_timeout: Duration::from_secs(20),
                connects_to: vec!["example.com:443:host-47.example.com:443".to_string()],
                cookie_input_file: Some("cookie_file".to_string()),
                follow_location: true,
                follow_location_trusted: false,
                http_version: RequestedHttpVersion::Http10,
                insecure: true,
                ip_resolve: IpResolve::IpV6,
                max_redirect: Some(10),
                netrc: false,
                netrc_file: Some("/var/run/netrc".to_string()),
                netrc_optional: true,
                path_as_is: true,
                proxy: Some("localhost:3128".to_string()),
                no_proxy: None,
                resolves: vec![
                    "foo.com:80:192.168.0.1".to_string(),
                    "bar.com:443:127.0.0.1".to_string()
                ],
                retry: Retry::None,
                ssl_no_revoke: false,
                timeout: Duration::from_secs(10),
                unix_socket: Some("/var/run/example.sock".to_string()),
                user: Some("user:password".to_string()),
                user_agent: Some("my-useragent".to_string()),
                verbosity: None,
            }
            .curl_args(),
            [
                "--compressed",
                "--connect-timeout",
                "20",
                "--connect-to",
                "example.com:443:host-47.example.com:443",
                "--cookie",
                "cookie_file",
                "--http1.0",
                "--insecure",
                "--ipv6",
                "--location",
                "--max-redirs",
                "10",
                "--netrc-file",
                "'/var/run/netrc'",
                "--netrc-optional",
                "--path-as-is",
                "--proxy",
                "'localhost:3128'",
                "--resolve",
                "foo.com:80:192.168.0.1",
                "--resolve",
                "bar.com:443:127.0.0.1",
                "--timeout",
                "10",
                "--unix-socket",
                "'/var/run/example.sock'",
                "--user",
                "'user:password'",
                "--user-agent",
                "'my-useragent'",
            ]
            .map(|a| a.to_string())
        );
    }
}
