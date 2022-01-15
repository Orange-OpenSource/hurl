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
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct ClientOptions {
    pub cacert_file: Option<String>,
    pub follow_location: bool,
    pub max_redirect: Option<usize>,
    pub cookie_input_file: Option<String>,
    pub proxy: Option<String>,
    pub no_proxy: Option<String>,
    pub verbose: bool,
    pub insecure: bool,
    pub timeout: Duration,
    pub connect_timeout: Duration,
    pub user: Option<String>,
    pub compressed: bool,
    pub context_dir: String,
}

impl Default for ClientOptions {
    fn default() -> Self {
        ClientOptions {
            cacert_file: None,
            follow_location: false,
            max_redirect: Some(50),
            cookie_input_file: None,
            proxy: None,
            no_proxy: None,
            verbose: false,
            insecure: false,
            timeout: Duration::from_secs(300),
            connect_timeout: Duration::from_secs(300),
            user: None,
            compressed: false,
            context_dir: ".".to_string(),
        }
    }
}

impl ClientOptions {
    pub fn curl_args(&self) -> Vec<String> {
        let mut arguments = vec![];

        if let Some(cacert_file) = self.cacert_file.clone() {
            arguments.push("--cacert".to_string());
            arguments.push(cacert_file);
        }

        if self.compressed {
            arguments.push("--compressed".to_string());
        }

        if self.connect_timeout != ClientOptions::default().connect_timeout {
            arguments.push("--connect-timeout".to_string());
            arguments.push(self.connect_timeout.as_secs().to_string());
        }

        if let Some(cookie_file) = self.cookie_input_file.clone() {
            arguments.push("--cookie".to_string());
            arguments.push(cookie_file);
        }

        if self.insecure {
            arguments.push("--insecure".to_string());
        }
        if self.follow_location {
            arguments.push("-L".to_string());
        }
        if self.max_redirect != ClientOptions::default().max_redirect {
            let max_redirect = match self.max_redirect {
                None => -1,
                Some(n) => n as i32,
            };
            arguments.push("--max-redirs".to_string());
            arguments.push(max_redirect.to_string());
        }
        if let Some(proxy) = self.proxy.clone() {
            arguments.push("--proxy".to_string());
            arguments.push(format!("'{}'", proxy));
        }
        if self.timeout != ClientOptions::default().timeout {
            arguments.push("--timeout".to_string());
            arguments.push(self.timeout.as_secs().to_string());
        }
        if let Some(user) = self.user.clone() {
            arguments.push("--user".to_string());
            arguments.push(format!("'{}'", user));
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
                follow_location: true,
                max_redirect: Some(10),
                cookie_input_file: Some("cookie_file".to_string()),
                proxy: Some("localhost:3128".to_string()),
                no_proxy: None,
                verbose: true,
                insecure: true,
                timeout: Duration::new(10, 0),
                connect_timeout: Duration::new(20, 0),
                user: Some("user:password".to_string()),
                compressed: true,
                context_dir: "".to_string()
            }
            .curl_args(),
            [
                "--compressed".to_string(),
                "--connect-timeout".to_string(),
                "20".to_string(),
                "--cookie".to_string(),
                "cookie_file".to_string(),
                "--insecure".to_string(),
                "-L".to_string(),
                "--max-redirs".to_string(),
                "10".to_string(),
                "--proxy".to_string(),
                "'localhost:3128'".to_string(),
                "--timeout".to_string(),
                "10".to_string(),
                "--user".to_string(),
                "'user:password'".to_string()
            ]
        );
    }
}
