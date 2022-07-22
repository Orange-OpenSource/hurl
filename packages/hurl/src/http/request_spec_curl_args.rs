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

use super::core::*;
use super::RequestSpec;
use crate::http::*;
use std::collections::HashMap;

impl RequestSpec {
    ///
    /// return request as curl arguments
    /// It does not contain the requests cookies (they will be accessed from the client)
    ///
    pub fn curl_args(&self, context_dir: &ContextDir) -> Vec<String> {
        let querystring = if self.querystring.is_empty() {
            "".to_string()
        } else {
            let params = self
                .querystring
                .iter()
                .map(|p| p.curl_arg_escape())
                .collect::<Vec<String>>();
            params.join("&")
        };
        let url = if querystring.as_str() == "" {
            self.url.to_string()
        } else if self.url.to_string().contains('?') {
            format!("{}&{}", self.url, querystring)
        } else {
            format!("{}?{}", self.url, querystring)
        };
        let mut arguments = vec![format!("'{}'", url)];

        let data =
            !self.multipart.is_empty() || !self.form.is_empty() || !self.body.bytes().is_empty();
        arguments.append(&mut self.method.curl_args(data));

        for header in self.headers.clone() {
            arguments.append(&mut header.curl_args());
        }

        let has_explicit_content_type = self
            .headers
            .iter()
            .map(|h| h.name.clone())
            .any(|n| n.as_str() == "Content-Type");
        if !has_explicit_content_type {
            if let Some(content_type) = self.content_type.clone() {
                if content_type.as_str() != "application/x-www-form-urlencoded"
                    && content_type.as_str() != "multipart/form-data"
                {
                    arguments.push("-H".to_string());
                    arguments.push(format!("'Content-Type: {}'", content_type));
                }
            } else if !self.body.bytes().is_empty() {
                match self.body.clone() {
                    Body::Text(_) => {
                        arguments.push("-H".to_string());
                        arguments.push("'Content-Type:'".to_string())
                    }
                    Body::Binary(_) => {
                        arguments.push("-H".to_string());
                        arguments.push("'Content-Type: application/octet-stream'".to_string())
                    }
                    Body::File(_, _) => {
                        arguments.push("-H".to_string());
                        arguments.push("'Content-Type:'".to_string())
                    }
                }
            }
        }

        for param in self.form.clone() {
            arguments.push("--data".to_string());
            arguments.push(format!("'{}'", param.curl_arg_escape()));
        }
        for param in self.multipart.clone() {
            arguments.push("-F".to_string());
            arguments.push(format!("'{}'", param.curl_arg(context_dir)));
        }

        if !self.body.bytes().is_empty() {
            arguments.push("--data".to_string());
            arguments.push(self.body.curl_arg(context_dir));
        }
        arguments
    }
}

fn encode_byte(b: u8) -> String {
    format!("\\x{:02x}", b)
}

fn encode_bytes(b: Vec<u8>) -> String {
    b.iter().map(|b| encode_byte(*b)).collect()
}

impl Method {
    pub fn curl_args(&self, data: bool) -> Vec<String> {
        match self {
            Method::Get => {
                if data {
                    vec!["-X".to_string(), "GET".to_string()]
                } else {
                    vec![]
                }
            }
            Method::Head => vec!["-X".to_string(), "HEAD".to_string()],
            Method::Post => {
                if data {
                    vec![]
                } else {
                    vec!["-X".to_string(), "POST".to_string()]
                }
            }
            Method::Put => vec!["-X".to_string(), "PUT".to_string()],
            Method::Delete => vec!["-X".to_string(), "DELETE".to_string()],
            Method::Connect => vec!["-X".to_string(), "CONNECT".to_string()],
            Method::Options => vec!["-X".to_string(), "OPTIONS".to_string()],
            Method::Trace => vec!["-X".to_string(), "TRACE".to_string()],
            Method::Patch => vec!["-X".to_string(), "PATCH".to_string()],
        }
    }
}

impl Header {
    pub fn curl_args(&self) -> Vec<String> {
        let name = self.name.clone();
        let value = self.value.clone();
        vec![
            "-H".to_string(),
            encode_shell_string(format!("{}: {}", name, value).as_str()),
        ]
    }
}

impl Param {
    pub fn curl_arg_escape(&self) -> String {
        let name = self.name.clone();
        let value = escape_url(self.value.clone());
        format!("{}={}", name, value)
    }

    pub fn curl_arg(&self) -> String {
        let name = self.name.clone();
        let value = self.value.clone();
        format!("{}={}", name, value)
    }
}

impl MultipartParam {
    pub fn curl_arg(&self, context_dir: &ContextDir) -> String {
        match self {
            MultipartParam::Param(param) => param.curl_arg(),
            MultipartParam::FileParam(FileParam {
                name,
                filename,
                content_type,
                ..
            }) => {
                let path = context_dir.get_path(filename);
                let value = format!("@{};type={}", path.to_str().unwrap(), content_type);
                format!("{}={}", name, value)
            }
        }
    }
}

impl Body {
    pub fn curl_arg(&self, context_dir: &ContextDir) -> String {
        match self.clone() {
            Body::Text(s) => encode_shell_string(&s),
            Body::Binary(bytes) => format!("$'{}'", encode_bytes(bytes)),
            Body::File(_, filename) => {
                let path = context_dir.get_path(&filename);
                format!("'@{}'", path.to_str().unwrap())
            }
        }
    }
}

fn escape_url(s: String) -> String {
    percent_encoding::percent_encode(s.as_bytes(), percent_encoding::NON_ALPHANUMERIC).to_string()
}

fn encode_shell_string(s: &str) -> String {
    // $'...' form will be used to encode escaped sequence
    if escape_mode(s) {
        let escaped = escape_string(s);
        format!("$'{}'", escaped)
    } else {
        format!("'{}'", s)
    }
}

// the shell string must be in escaped mode ($'...')
// if it contains \n, \t or '
fn escape_mode(s: &str) -> bool {
    for c in s.chars() {
        if c == '\n' || c == '\t' || c == '\'' {
            return true;
        }
    }
    false
}

fn escape_string(s: &str) -> String {
    let mut escaped_sequences = HashMap::new();
    escaped_sequences.insert('\n', "\\n");
    escaped_sequences.insert('\t', "\\t");
    escaped_sequences.insert('\'', "\\'");
    escaped_sequences.insert('\\', "\\\\");

    let mut escaped = "".to_string();
    for c in s.chars() {
        match escaped_sequences.get(&c) {
            None => escaped.push(c),
            Some(escaped_seq) => escaped.push_str(escaped_seq),
        }
    }
    escaped
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_encode_byte() {
        assert_eq!(encode_byte(1), "\\x01".to_string());
        assert_eq!(encode_byte(32), "\\x20".to_string());
    }

    #[test]
    fn method_curl_args() {
        assert!(Method::Get.curl_args(false).is_empty());
        assert_eq!(
            Method::Get.curl_args(true),
            vec!["-X".to_string(), "GET".to_string()]
        );

        assert_eq!(
            Method::Post.curl_args(false),
            vec!["-X".to_string(), "POST".to_string()]
        );
        assert!(Method::Post.curl_args(true).is_empty());

        assert_eq!(
            Method::Put.curl_args(false),
            vec!["-X".to_string(), "PUT".to_string()]
        );
        assert_eq!(
            Method::Put.curl_args(true),
            vec!["-X".to_string(), "PUT".to_string()]
        );
    }

    #[test]
    fn header_curl_args() {
        assert_eq!(
            Header {
                name: "Host".to_string(),
                value: "example.com".to_string(),
            }
            .curl_args(),
            vec!["-H".to_string(), "'Host: example.com'".to_string()]
        );
        assert_eq!(
            Header {
                name: "If-Match".to_string(),
                value: "\"e0023aa4e\"".to_string(),
            }
            .curl_args(),
            vec!["-H".to_string(), "'If-Match: \"e0023aa4e\"'".to_string()]
        );
    }

    #[test]
    fn param_curl_args() {
        assert_eq!(
            Param {
                name: "param1".to_string(),
                value: "value1".to_string(),
            }
            .curl_arg(),
            "param1=value1".to_string()
        );
        assert_eq!(
            Param {
                name: "param2".to_string(),
                value: "".to_string(),
            }
            .curl_arg(),
            "param2=".to_string()
        );
        assert_eq!(
            Param {
                name: "param3".to_string(),
                value: "a=b".to_string(),
            }
            .curl_arg_escape(),
            "param3=a%3Db".to_string()
        );
        assert_eq!(
            Param {
                name: "param4".to_string(),
                value: "1,2,3".to_string(),
            }
            .curl_arg_escape(),
            "param4=1%2C2%2C3".to_string()
        );
    }

    #[test]
    fn requests_curl_args() {
        let context_dir = &ContextDir::default();
        assert_eq!(
            hello_http_request().curl_args(context_dir),
            vec!["'http://localhost:8000/hello'".to_string()]
        );
        assert_eq!(
            custom_http_request().curl_args(context_dir),
            vec![
                "'http://localhost/custom'".to_string(),
                "-H".to_string(),
                "'User-Agent: iPhone'".to_string(),
                "-H".to_string(),
                "'Foo: Bar'".to_string(),
            ]
        );
        assert_eq!(
            query_http_request().curl_args(context_dir),
            vec![
                "'http://localhost:8000/querystring-params?param1=value1&param2=a%20b'".to_string()
            ]
        );
        assert_eq!(
            form_http_request().curl_args(context_dir),
            vec![
                "'http://localhost/form-params'".to_string(),
                "-H".to_string(),
                "'Content-Type: application/x-www-form-urlencoded'".to_string(),
                "--data".to_string(),
                "'param1=value1'".to_string(),
                "--data".to_string(),
                "'param2=a%20b'".to_string(),
            ]
        );
    }

    #[test]
    fn test_encode_body() {
        let current_dir = Path::new("/tmp");
        let file_root = Path::new("/tmp");
        let context_dir = ContextDir::new(current_dir, file_root);
        assert_eq!(
            Body::Text("hello".to_string()).curl_arg(&context_dir),
            "'hello'".to_string()
        );

        if cfg!(unix) {
            assert_eq!(
                Body::File(vec![], "filename".to_string()).curl_arg(&context_dir),
                "'@/tmp/filename'".to_string()
            );
        }

        assert_eq!(
            Body::Binary(vec![1, 2, 3]).curl_arg(&context_dir),
            "$'\\x01\\x02\\x03'".to_string()
        );
    }

    #[test]
    fn test_encode_shell_string() {
        assert_eq!(encode_shell_string("hello"), "'hello'");
        assert_eq!(encode_shell_string("\\n"), "'\\n'");
        assert_eq!(encode_shell_string("'"), "$'\\''");
        assert_eq!(encode_shell_string("\\'"), "$'\\\\\\''");
        assert_eq!(encode_shell_string("\n"), "$'\\n'");
    }

    #[test]
    fn test_escape_string() {
        assert_eq!(escape_string("hello"), "hello");
        assert_eq!(escape_string("\\n"), "\\\\n");
        assert_eq!(escape_string("'"), "\\'");
        assert_eq!(escape_string("\\'"), "\\\\\\'");
        assert_eq!(escape_string("\n"), "\\n");
    }

    #[test]
    fn test_escape_mode() {
        assert!(!escape_mode("hello"));
        assert!(!escape_mode("\\"));
        assert!(escape_mode("'"));
        assert!(escape_mode("\n"));
    }
}
