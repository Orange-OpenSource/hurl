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

use core::fmt;
use std::path::Path;

use super::core::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RequestSpec {
    pub method: Method,
    pub url: String,
    pub headers: Vec<Header>,
    pub querystring: Vec<Param>,
    pub form: Vec<Param>,
    pub multipart: Vec<MultipartParam>,
    pub cookies: Vec<RequestCookie>,
    pub body: Body,
    pub content_type: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Method {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Patch,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MultipartParam {
    Param(Param),
    FileParam(FileParam),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FileParam {
    pub name: String,
    pub filename: String,
    pub data: Vec<u8>,
    pub content_type: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Body {
    Text(String),
    Binary(Vec<u8>),
    File(Vec<u8>, String),
}

impl Body {
    pub fn bytes(&self) -> Vec<u8> {
        match self {
            Body::Text(s) => s.as_bytes().to_vec(),
            Body::Binary(bs) => bs.clone(),
            Body::File(bs, _) => bs.clone(),
        }
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Method::Get => "GET",
            Method::Head => "HEAD",
            Method::Post => "POST",
            Method::Put => "PUT",
            Method::Delete => "DELETE",
            Method::Connect => "CONNECT",
            Method::Options => "OPTIONS",
            Method::Trace => "TRACE",
            Method::Patch => "PATCH",
        };
        write!(f, "{}", value)
    }
}

impl fmt::Display for MultipartParam {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MultipartParam::Param(param) => write!(f, "{}", param),
            MultipartParam::FileParam(param) => write!(f, "{}", param),
        }
    }
}

impl fmt::Display for FileParam {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}: file,{}; {}",
            self.name, self.filename, self.content_type
        )
    }
}

impl RequestSpec {
    ///
    /// return request as curl arguments
    /// It does not contain the requests cookies (they will be accessed from the client)
    ///
    pub fn curl_args(&self, context_dir: &Path) -> Vec<String> {
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
            match self.body.clone() {
                Body::Text(s) => {
                    let prefix = if s.contains('\n') { "$" } else { "" };
                    arguments.push(format!(
                        "{}'{}'",
                        prefix,
                        s.replace('\\', "\\\\").replace('\n', "\\n")
                    ))
                }
                Body::Binary(bytes) => arguments.push(format!("$'{}'", encode_bytes(bytes))),
                Body::File(_, filename) => {
                    let path = Path::new(&filename);
                    let path = context_dir.join(path);
                    arguments.push(format!("'@{}'", path.to_str().unwrap()))
                }
            }
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
            encode_value(format!("{}: {}", name, value)),
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
    pub fn curl_arg(&self, context_dir: &Path) -> String {
        match self {
            MultipartParam::Param(param) => param.curl_arg(),
            MultipartParam::FileParam(FileParam {
                name,
                filename,
                content_type,
                ..
            }) => {
                let path = Path::new(&filename);
                let path = context_dir.join(path);
                let value = format!("@{};type={}", path.to_str().unwrap(), content_type);
                format!("{}={}", name, value)
            }
        }
    }
}

fn escape_single_quote(s: String) -> String {
    s.chars()
        .map(|c| {
            if c == '\'' {
                "\\'".to_string()
            } else {
                c.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join("")
}

fn escape_url(s: String) -> String {
    percent_encoding::percent_encode(s.as_bytes(), percent_encoding::NON_ALPHANUMERIC).to_string()
}

// special encoding for the shell
// $'...'
fn encode_value(s: String) -> String {
    if s.contains('\'') {
        let s = escape_single_quote(s);
        format!("$'{}'", s)
    } else {
        format!("'{}'", s)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn hello_http_request() -> RequestSpec {
        RequestSpec {
            method: Method::Get,
            url: "http://localhost:8000/hello".to_string(),
            querystring: vec![],
            headers: vec![],
            cookies: vec![],
            body: Body::Binary(vec![]),
            multipart: vec![],
            form: vec![],
            content_type: None,
        }
    }

    pub fn custom_http_request() -> RequestSpec {
        RequestSpec {
            method: Method::Get,
            url: "http://localhost/custom".to_string(),
            querystring: vec![],
            headers: vec![
                Header {
                    name: String::from("User-Agent"),
                    value: String::from("iPhone"),
                },
                Header {
                    name: String::from("Foo"),
                    value: String::from("Bar"),
                },
            ],
            cookies: vec![
                RequestCookie {
                    name: String::from("theme"),
                    value: String::from("light"),
                },
                RequestCookie {
                    name: String::from("sessionToken"),
                    value: String::from("abc123"),
                },
            ],
            body: Body::Binary(vec![]),
            multipart: vec![],
            form: vec![],
            content_type: None,
        }
    }

    pub fn query_http_request() -> RequestSpec {
        RequestSpec {
            method: Method::Get,
            url: "http://localhost:8000/querystring-params".to_string(),
            querystring: vec![
                Param {
                    name: String::from("param1"),
                    value: String::from("value1"),
                },
                Param {
                    name: String::from("param2"),
                    value: String::from("a b"),
                },
            ],
            headers: vec![],
            cookies: vec![],
            body: Body::Binary(vec![]),
            multipart: vec![],
            form: vec![],
            content_type: None,
        }
    }

    pub fn form_http_request() -> RequestSpec {
        RequestSpec {
            method: Method::Post,
            url: "http://localhost/form-params".to_string(),
            querystring: vec![],
            headers: vec![Header {
                name: String::from("Content-Type"),
                value: String::from("application/x-www-form-urlencoded"),
            }],
            cookies: vec![],
            body: Body::Binary(vec![]),
            multipart: vec![],
            form: vec![
                Param {
                    name: String::from("param1"),
                    value: String::from("value1"),
                },
                Param {
                    name: String::from("param2"),
                    value: String::from("a b"),
                },
            ],
            content_type: Some("multipart/form-data".to_string()),
        }
    }

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
                value: "example.com".to_string()
            }
            .curl_args(),
            vec!["-H".to_string(), "'Host: example.com'".to_string()]
        );
        assert_eq!(
            Header {
                name: "If-Match".to_string(),
                value: "\"e0023aa4e\"".to_string()
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
                value: "value1".to_string()
            }
            .curl_arg(),
            "param1=value1".to_string()
        );
        assert_eq!(
            Param {
                name: "param2".to_string(),
                value: "".to_string()
            }
            .curl_arg(),
            "param2=".to_string()
        );
        assert_eq!(
            Param {
                name: "param3".to_string(),
                value: "a=b".to_string()
            }
            .curl_arg_escape(),
            "param3=a%3Db".to_string()
        );
        assert_eq!(
            Param {
                name: "param4".to_string(),
                value: "1,2,3".to_string()
            }
            .curl_arg_escape(),
            "param4=1%2C2%2C3".to_string()
        );
    }

    #[test]
    fn requests_curl_args() {
        assert_eq!(
            hello_http_request().curl_args(Path::new("")),
            vec!["'http://localhost:8000/hello'".to_string()]
        );
        assert_eq!(
            custom_http_request().curl_args(Path::new("")),
            vec![
                "'http://localhost/custom'".to_string(),
                "-H".to_string(),
                "'User-Agent: iPhone'".to_string(),
                "-H".to_string(),
                "'Foo: Bar'".to_string(),
            ]
        );
        assert_eq!(
            query_http_request().curl_args(Path::new("")),
            vec![
                "'http://localhost:8000/querystring-params?param1=value1&param2=a%20b'".to_string()
            ]
        );
        assert_eq!(
            form_http_request().curl_args(Path::new("")),
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
    fn test_encode_value() {
        assert_eq!(
            encode_value("Header1: x".to_string()),
            "'Header1: x'".to_string()
        );
        assert_eq!(
            encode_value("Header1: '".to_string()),
            "$'Header1: \\''".to_string()
        );
    }
}
