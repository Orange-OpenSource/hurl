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
#[allow(unused)]
use std::io::prelude::*;

use crate::core::ast::*;
use crate::core::common::Value;
use crate::http;

use super::core::Error;

impl Request {
    pub fn eval(
        self,
        variables: &HashMap<String, Value>,
        context_dir: String,
    ) -> Result<http::Request, Error> {
        let method = self.method.clone().eval();

        let url = self.clone().url.eval(&variables)?;

        // headers
        let mut headers: Vec<http::Header> = vec![];
        for header in self.clone().headers {
            let name = header.key.value;
            let value = header.value.eval(variables)?;
            headers.push(http::Header { name, value });
        }

        let mut querystring: Vec<http::Param> = vec![];
        for param in self.clone().querystring_params() {
            let name = param.key.value;
            let value = param.value.eval(variables)?;
            querystring.push(http::Param { name, value });
        }

        let mut form: Vec<http::Param> = vec![];
        for param in self.clone().form_params() {
            let name = param.key.value;
            let value = param.value.eval(variables)?;
            form.push(http::Param { name, value });
        }
        //        if !self.clone().form_params().is_empty() {
        //            headers.push(http::core::Header {
        //                name: String::from("Content-Type"),
        //                value: String::from("application/x-www-form-urlencoded"),
        //            });
        //        }

        let mut cookies = vec![];
        for cookie in self.clone().cookies() {
            let cookie = http::RequestCookie {
                name: cookie.clone().name.value,
                value: cookie.clone().value.value,
            };
            cookies.push(cookie);
        }

        let bytes = match self.clone().body {
            Some(body) => body.eval(variables, context_dir.clone())?,
            None => vec![],
        };

        let mut multipart = vec![];
        for multipart_param in self.clone().multipart_form_data() {
            let param = multipart_param.eval(variables, context_dir.clone())?;
            multipart.push(param);
        }

        let content_type = if !form.is_empty() {
            Some("application/x-www-form-urlencoded".to_string())
        } else if !multipart.is_empty() {
            Some("multipart/form-data".to_string())
        } else if let Some(Body {
            value: Bytes::Json { .. },
            ..
        }) = self.body
        {
            Some("application/json".to_string())
        } else {
            None
        };

        // add implicit content type
        //        if self.content_type().is_none() {
        //            if let Some(body) = self.body {
        //                if let Bytes::Json { .. } = body.value {
        //                    headers.push(http::core::Header {
        //                        name: String::from("Content-Type"),
        //                        value: String::from("application/json"),
        //                    });
        //                }
        //            }
        //        }

        Ok(http::Request {
            method,
            url,
            querystring,
            headers,
            cookies,
            body: bytes,
            multipart,
            form,
            content_type,
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
    fn eval(self) -> http::Method {
        match self {
            Method::Get => http::Method::Get,
            Method::Head => http::Method::Head,
            Method::Post => http::Method::Post,
            Method::Put => http::Method::Put,
            Method::Delete => http::Method::Delete,
            Method::Connect => http::Method::Connect,
            Method::Options => http::Method::Options,
            Method::Trace => http::Method::Trace,
            Method::Patch => http::Method::Patch,
        }
    }
}

//pub fn split_url(url: String) -> (String, Vec<http::Param>) {
//    match url.find('?') {
//        None => (url, vec![]),
//        Some(index) => {
//            let (url, params) = url.split_at(index);
//            let params: Vec<http::Param> = params[1..].split('&')
//                .map(|s| {
//                    match s.find('=') {
//                        None => http::Param { name: s.to_string(), value: String::from("") },
//                        Some(index) => {
//                            let (name, value) = s.split_at(index);
//                            http::Param { name: name.to_string(), value: value[1..].to_string() }
//                        }
//                    }
//                })
//                .collect();
//
//            (url.to_string(), params)
//        }
//    }
//}

#[cfg(test)]
mod tests {
    use crate::core::common::SourceInfo;

    use super::super::core::RunnerError;
    use super::*;

    pub fn whitespace() -> Whitespace {
        Whitespace {
            value: String::from(" "),
            source_info: SourceInfo::init(0, 0, 0, 0),
        }
    }

    pub fn hello_request() -> Request {
        let line_terminator = LineTerminator {
            space0: whitespace(),
            comment: None,
            newline: whitespace(),
        };
        Request {
            line_terminators: vec![],
            space0: whitespace(),
            method: Method::Get,
            space1: whitespace(),
            url: Template {
                elements: vec![
                    TemplateElement::Expression(Expr {
                        space0: whitespace(),
                        variable: Variable {
                            name: String::from("base_url"),
                            source_info: SourceInfo::init(1, 7, 1, 15),
                        },
                        space1: whitespace(),
                    }),
                    TemplateElement::String {
                        value: String::from("/hello"),
                        encoded: String::from("/hello"),
                    },
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
        let line_terminator = LineTerminator {
            space0: whitespace(),
            comment: None,
            newline: whitespace(),
        };
        KeyValue {
            line_terminators: vec![],
            space0: whitespace(),
            key,
            space1: whitespace(),
            space2: whitespace(),
            value,
            line_terminator0: line_terminator,
        }
    }

    pub fn query_request() -> Request {
        let line_terminator = LineTerminator {
            space0: whitespace(),
            comment: None,
            newline: whitespace(),
        };
        Request {
            line_terminators: vec![],
            space0: whitespace(),
            method: Method::Get,
            space1: whitespace(),
            url: Template {
                elements: vec![TemplateElement::String {
                    value: String::from("http://localhost:8000/querystring-params"),
                    encoded: String::from("http://localhost:8000/querystring-params"),
                }],
                quotes: false,
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
            line_terminator0: line_terminator.clone(),
            headers: vec![],
            sections: vec![Section {
                line_terminators: vec![],
                space0: whitespace(),
                line_terminator0: line_terminator,
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
                            elements: vec![TemplateElement::Expression(Expr {
                                space0: whitespace(),
                                variable: Variable {
                                    name: String::from("param1"),
                                    source_info: SourceInfo::init(1, 7, 1, 15),
                                },
                                space1: whitespace(),
                            })],
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
                            elements: vec![TemplateElement::String {
                                value: "a b".to_string(),
                                encoded: "a b".to_string(),
                            }],
                            source_info: SourceInfo::init(0, 0, 0, 0),
                        },
                    ),
                ]),
                source_info: SourceInfo::init(0, 0, 0, 0),
            }],
            body: None,
            source_info: SourceInfo::init(0, 0, 0, 0),
        }
    }

    #[test]
    pub fn test_error_variable() {
        let variables = HashMap::new();
        let error = hello_request()
            .eval(&variables, "current_dir".to_string())
            .err()
            .unwrap();
        assert_eq!(error.source_info, SourceInfo::init(1, 7, 1, 15));
        assert_eq!(
            error.inner,
            RunnerError::TemplateVariableNotDefined {
                name: String::from("base_url")
            }
        );
    }

    #[test]
    pub fn test_hello_request() {
        let mut variables = HashMap::new();
        variables.insert(
            String::from("base_url"),
            Value::String(String::from("http://localhost:8000")),
        );
        let http_request = hello_request()
            .eval(&variables, "current_dir".to_string())
            .unwrap();
        assert_eq!(http_request, http::hello_http_request());
    }

    #[test]
    pub fn test_query_request() {
        let mut variables = HashMap::new();
        variables.insert(
            String::from("param1"),
            Value::String(String::from("value1")),
        );
        let http_request = query_request()
            .eval(&variables, "current_dir".to_string())
            .unwrap();
        assert_eq!(http_request, http::query_http_request());
    }
}
