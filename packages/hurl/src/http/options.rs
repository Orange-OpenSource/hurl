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

#[derive(Debug, Clone)]
pub struct ClientOptions {
    pub cacert_file: Option<String>,
    pub client_cert_file: Option<String>,
    pub client_key_file: Option<String>,
    pub compressed: bool,
    pub connect_timeout: Duration,
    pub connects_to: Vec<String>,
    pub cookie_input_file: Option<String>,
    pub follow_location: bool,
    pub insecure: bool,
    pub max_redirect: Option<usize>,
    pub no_proxy: Option<String>,
    pub proxy: Option<String>,
    pub resolves: Vec<String>,
    pub retry_max_count: Option<usize>,
    pub ssl_no_revoke: bool,
    pub timeout: Duration,
    pub user: Option<String>,
    pub user_agent: Option<String>,
    pub verbosity: Option<Verbosity>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Verbosity {
    Verbose,
    VeryVerbose,
}

impl Default for ClientOptions {
    fn default() -> Self {
        ClientOptions {
            cacert_file: None,
            client_cert_file: None,
            client_key_file: None,
            compressed: false,
            connect_timeout: Duration::from_secs(300),
            connects_to: vec![],
            cookie_input_file: None,
            follow_location: false,
            insecure: false,
            max_redirect: Some(50),
            no_proxy: None,
            proxy: None,
            resolves: vec![],
            retry_max_count: Some(10),
            ssl_no_revoke: false,
            timeout: Duration::from_secs(300),
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
        if self.insecure {
            arguments.push("--insecure".to_string());
        }
        if self.follow_location {
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
                cacert_file: None,
                client_cert_file: None,
                client_key_file: None,
                connects_to: vec!["example.com:443:host-47.example.com:443".to_string()],
                follow_location: true,
                max_redirect: Some(10),
                cookie_input_file: Some("cookie_file".to_string()),
                proxy: Some("localhost:3128".to_string()),
                no_proxy: None,
                verbosity: None,
                insecure: true,
                resolves: vec![
                    "foo.com:80:192.168.0.1".to_string(),
                    "bar.com:443:127.0.0.1".to_string()
                ],
                retry_max_count: Some(10),
                ssl_no_revoke: false,
                timeout: Duration::from_secs(10),
                connect_timeout: Duration::from_secs(20),
                user: Some("user:password".to_string()),
                user_agent: Some("my-useragent".to_string()),
                compressed: true,
            }
            .curl_args(),
            [
                "--compressed".to_string(),
                "--connect-timeout".to_string(),
                "20".to_string(),
                "--connect-to".to_string(),
                "example.com:443:host-47.example.com:443".to_string(),
                "--cookie".to_string(),
                "cookie_file".to_string(),
                "--insecure".to_string(),
                "--location".to_string(),
                "--max-redirs".to_string(),
                "10".to_string(),
                "--proxy".to_string(),
                "'localhost:3128'".to_string(),
                "--resolve".to_string(),
                "foo.com:80:192.168.0.1".to_string(),
                "--resolve".to_string(),
                "bar.com:443:127.0.0.1".to_string(),
                "--timeout".to_string(),
                "10".to_string(),
                "--user".to_string(),
                "'user:password'".to_string(),
                "--user-agent".to_string(),
                "'my-useragent'".to_string(),
            ]
        );
    }
}
