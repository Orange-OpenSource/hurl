/*
 * hurl (https://hurl.dev)
 * Copyright (C) 2020 Orange
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
extern crate libxml;
extern crate serde_json;
extern crate url as external_url;

use std::collections::HashMap;
use std::path::Path;

use crate::core::ast::*;
use crate::core::common::Value;
use crate::http;

use super::core::{Error, RunnerError};

impl Request {
    pub fn eval(self,
                variables: &HashMap<String, Value>,
                context_dir: String,
    ) -> Result<http::request::Request, Error> {
        let method = self.method.clone().eval();
        let url = self.clone().url.eval(&variables)?;
        let mut querystring: Vec<http::core::Param> = vec![];

        // query string from url
        // parse url string
        let (url, params) = match external_url::Url::parse(url.as_str()) {
            Err(_) => {
                return Err(Error {
                    source_info: self.clone().url.source_info,
                    inner: RunnerError::InvalidURL(url),
                    assert: false,
                });
            }
            Ok(u) => {
                let url = http::core::Url {
                    scheme: u.scheme().to_string(),
                    host: u.host_str().unwrap().to_string(),
                    port: u.port(),
                    path: u.path().to_string(),
                    query_string: if let Some(s) = u.query() { s.to_string() } else { "".to_string() },
                };
                let params: Vec<http::core::Param> = vec![];
                (url, params)
            }
        };
        for param in params {
            querystring.push(param);
        }
        for param in self.clone().querystring_params() {
            let name = param.key.value;
            let value = param.value.eval(variables)?;
            querystring.push(http::core::Param { name, value });
        }

        // headers
        let mut headers: Vec<http::core::Header> = vec![];
        for header in self.clone().headers {
            let name = header.key.value;
            let value = header.value.eval(variables)?;
            headers.push(http::core::Header {
                name,
                value,
            });
        }


        // add cookies
        //let host = url.host.as_str();
        let mut cookies = vec![];
        // TODO cookie from header
        for cookie in self.clone().cookies() {
            let cookie = http::cookie::ResponseCookie {
                name: cookie.clone().name.value,
                value: cookie.clone().value.value,
                max_age: None,
                domain: Some(url.clone().host),
                path: None,
                secure: None,
                http_only: None,
                expires: None,
                same_site: None,
            };
            //headers.push(cookie.to_header());
            cookies.push(cookie);
        }


        if !self.clone().form_params().is_empty() {
            headers.push(http::core::Header {
                name: String::from("Content-Type"),
                value: String::from("application/x-www-form-urlencoded"),
            });
        }

        let bytes = match self.clone().body {
            Some(body) => body.eval(variables, context_dir.clone())?,
            None => {
                if !self.clone().form_params().is_empty() {
                    let mut params = vec![];
                    for param in self.clone().form_params() {
                        let name = param.key.value;
                        let value = param.value.eval(variables)?;
                        params.push(http::core::Param {
                            name,
                            value,
                        });
                    }

                    http::core::encode_form_params(params)
                } else {
                    vec![]
                }
            }
        };
        let mut multipart = vec![];
        for multipart_param in self.clone().multipart_form_data() {
            match multipart_param {
                MultipartParam::FileParam(FileParam { key, value: FileValue { filename, content_type, .. }, .. }) => {
                    let name = key.value;

                    let path = Path::new(filename.value.as_str());
                    let absolute_filename = if path.is_absolute() {
                        filename.clone().value
                    } else {
                        Path::new(context_dir.as_str()).join(filename.value.clone()).to_str().unwrap().to_string()
                    };

                    if !Path::new(&absolute_filename).exists() {
                        return Err(Error {
                            source_info: filename.source_info,
                            inner: RunnerError::FileReadAccess { value: filename.value.clone() },
                            assert: false,
                        });
                    }
                    multipart.push(http::request::MultipartParam::FileParam { name, filename: absolute_filename, content_type });
                }
                MultipartParam::Param(KeyValue { key, value, .. }) => {
                    let name = key.value;
                    let value = value.eval(variables)?;
                    multipart.push(http::request::MultipartParam::TextParam { name, value });
                }
            }
        }


        // add implicit content type
        if self.content_type().is_none() {
            if let Some(body) = self.body {
                if let Bytes::Json { .. } = body.value {
                    headers.push(http::core::Header {
                        name: String::from("Content-Type"),
                        value: String::from("application/json"),
                    });
                }
            }
        }

        Ok(http::request::Request {
            method,
            url,
            querystring,
            headers,
            cookies,
            body: bytes,
            multipart,
        })
    }

    pub fn content_type(&self) -> Option<Template> {
        for header in self.headers.clone() {
            if header.key.value.to_lowercase().as_str() == "content-type" {
                return Some(header.value);
            }
        }
        None
    }
}

impl Method {
    fn eval(self) -> http::request::Method {
        match self {
            Method::Get => http::request::Method::Get,
            Method::Head => http::request::Method::Head,
            Method::Post => http::request::Method::Post,
            Method::Put => http::request::Method::Put,
            Method::Delete => http::request::Method::Delete,
            Method::Connect => http::request::Method::Connect,
            Method::Options => http::request::Method::Options,
            Method::Trace => http::request::Method::Trace,
            Method::Patch => http::request::Method::Patch,
        }
    }
}

pub fn split_url(url: String) -> (String, Vec<http::core::Param>) {
    match url.find('?') {
        None => (url, vec![]),
        Some(index) => {
            let (url, params) = url.split_at(index);
            let params: Vec<http::core::Param> = params[1..].split('&')
                .map(|s| {
                    match s.find('=') {
                        None => http::core::Param { name: s.to_string(), value: String::from("") },
                        Some(index) => {
                            let (name, value) = s.split_at(index);
                            http::core::Param { name: name.to_string(), value: value[1..].to_string() }
                        }
                    }
                })
                .collect();

            (url.to_string(), params)
        }
    }
}

pub fn eval_url(s: String) -> Result<http::core::Url, RunnerError> {
    match url::Url::parse(s.as_str()) {
        Err(_) => Err(RunnerError::InvalidURL(s)),
        Ok(u) => Ok(http::core::Url {
            scheme: u.scheme().to_string(),
            host: u.host_str().unwrap().to_string(),
            port: u.port(),
            path: u.path().to_string(),

            query_string: match u.query() {
                None => "".to_string(),
                Some(s) => s.to_string()
            },
        })
    }
}


#[cfg(test)]
mod tests {
    use crate::core::common::SourceInfo;

    use super::*;

    pub fn hello_request() -> Request {

// GET {{base_url}}/hello
        let whitespace = Whitespace {
            value: String::from(" "),
            source_info: SourceInfo::init(0, 0, 0, 0),
        };
        let line_terminator = LineTerminator {
            space0: whitespace.clone(),
            comment: None,
            newline: whitespace.clone(),
        };
        Request {
            line_terminators: vec![],
            space0: whitespace.clone(),
            method: Method::Get,
            space1: whitespace.clone(),
            url: Template {
                elements: vec![
                    TemplateElement::Expression(Expr {
                        space0: whitespace.clone(),
                        variable: Variable {
                            name: String::from("base_url"),
                            source_info: SourceInfo::init(1, 7, 1, 15),
                        },
                        space1: whitespace.clone(),
                    }),
                    TemplateElement::String {
                        value: String::from("/hello"),
                        encoded: String::from("/hello"),
                    }
                ],
                quotes: false,
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
            line_terminator0: line_terminator.clone(),
            headers: vec![],
            sections: vec![],
            body: None,
            source_info: SourceInfo::init(0, 0, 0, 0),
        }
    }

    pub fn simple_key_value(key: EncodedString, value: Template) -> KeyValue {
        let whitespace = Whitespace {
            value: String::from(" "),
            source_info: SourceInfo::init(0, 0, 0, 0),
        };
        let line_terminator = LineTerminator {
            space0: whitespace.clone(),
            comment: None,
            newline: whitespace.clone(),
        };
        KeyValue {
            line_terminators: vec![],
            space0: whitespace.clone(),
            key,
            space1: whitespace.clone(),
            space2: whitespace.clone(),
            value,
            line_terminator0: line_terminator.clone(),
        }
    }

    pub fn query_request() -> Request {
        let whitespace = Whitespace {
            value: String::from(" "),
            source_info: SourceInfo::init(0, 0, 0, 0),
        };
        let line_terminator = LineTerminator {
            space0: whitespace.clone(),
            comment: None,
            newline: whitespace.clone(),
        };
        Request {
            line_terminators: vec![],
            space0: whitespace.clone(),
            method: Method::Get,
            space1: whitespace.clone(),
            url: Template {
                elements: vec![
                    TemplateElement::String {
                        value: String::from("http://localhost:8000/querystring-params"),
                        encoded: String::from("http://localhost:8000/querystring-params"),
                    }
                ],
                quotes: false,
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
            line_terminator0: line_terminator.clone(),
            headers: vec![],
            sections: vec![
                Section {
                    line_terminators: vec![],
                    space0: whitespace.clone(),
                    line_terminator0: line_terminator.clone(),
                    value: SectionValue::QueryParams(vec![
                        simple_key_value(
                            EncodedString {
                                quotes: false,
                                value: "param1".to_string(),
                                encoded: "param1".to_string(),
                                source_info: SourceInfo::init(0, 0, 0, 0),
                            },
                            Template {
                                quotes: false,
                                elements: vec![
                                    TemplateElement::Expression(Expr {
                                        space0: whitespace.clone(),
                                        variable: Variable {
                                            name: String::from("param1"),
                                            source_info: SourceInfo::init(1, 7, 1, 15),
                                        },
                                        space1: whitespace.clone(),
                                    })
                                ],
                                source_info: SourceInfo::init(0, 0, 0, 0),
                            },
                        ),
                        simple_key_value(
                            EncodedString {
                                quotes: false,
                                value: "param2".to_string(),
                                encoded: "param2".to_string(),
                                source_info: SourceInfo::init(0, 0, 0, 0),
                            },
                            Template {
                                quotes: false,
                                elements: vec![
                                    TemplateElement::String {
                                        value: "a b".to_string(),
                                        encoded: "a b".to_string(),
                                    }
                                ],
                                source_info: SourceInfo::init(0, 0, 0, 0),
                            },
                        )
                    ]),

                    source_info: SourceInfo::init(0, 0, 0, 0),
                },
            ],
            body: None,
            source_info: SourceInfo::init(0, 0, 0, 0),
        }
    }

    #[test]
    pub fn test_error_variable() {
        let variables = HashMap::new();
        let error = hello_request().eval(&variables, "current_dir".to_string()).err().unwrap();
        assert_eq!(error.source_info, SourceInfo::init(1, 7, 1, 15));
        assert_eq!(error.inner, RunnerError::TemplateVariableNotDefined { name: String::from("base_url") });
    }

    #[test]
    pub fn test_hello_request() {
        let mut variables = HashMap::new();
        // let cookies = HashMap::new();
        variables.insert(String::from("base_url"), Value::String(String::from("http://localhost:8000")));
        let http_request = hello_request().eval(&variables, "current_dir".to_string()).unwrap();
        assert_eq!(http_request, http::request::tests::hello_http_request());
    }

    #[test]
    pub fn test_query_request() {
        let mut variables = HashMap::new();
        //let cookies = HashMap::new();
        variables.insert(String::from("param1"), Value::String(String::from("value1")));
        let http_request = query_request().eval(&variables, "current_dir".to_string()).unwrap();
        assert_eq!(http_request, http::request::tests::query_http_request());
    }

    #[test]
    pub fn test_split_url() {
        assert_eq!(
            split_url(String::from("http://localhost:8000/hello")),
            (String::from("http://localhost:8000/hello"), vec![])
        );
        assert_eq!(
            split_url(String::from("http://localhost:8000/hello?param1=value1")),
            (String::from("http://localhost:8000/hello"), vec![http::core::Param { name: String::from("param1"), value: String::from("value1") }])
        );
    }

    #[test]
    pub fn test_eval_url() {
        assert_eq!(eval_url(String::from("xxx")).err().unwrap(), RunnerError::InvalidURL(String::from("xxx")));

        let url = eval_url(String::from("http://localhost:8000/querystring-params?param1=value1")).unwrap();
        assert_eq!(url.host, "localhost");
        assert_eq!(url.port, Some(8000));
// assert_eq!(url.querystring.unwrap(), String::from("param1=value1"));
    }
}
