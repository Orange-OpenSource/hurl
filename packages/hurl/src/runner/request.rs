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

use std::collections::HashMap;
#[allow(unused)]
use std::io::prelude::*;

use crate::http;
use crate::http::ContextDir;
use hurl_core::ast::*;

use super::body::eval_body;
use super::core::Error;
use super::template::eval_template;
use super::value::Value;
use crate::runner::multipart::eval_multipart_param;

pub fn eval_request(
    request: Request,
    variables: &HashMap<String, Value>,
    context_dir: &ContextDir,
) -> Result<http::RequestSpec, Error> {
    let method = eval_method(request.method.clone());

    let url = eval_template(&request.url, variables)?;

    // headers
    let mut headers: Vec<http::Header> = vec![];
    for header in request.clone().headers {
        let name = header.key.value;
        let value = eval_template(&header.value, variables)?;
        headers.push(http::Header { name, value });
    }

    if let Some(kv) = request.clone().basic_auth() {
        let value = eval_template(&kv.value, variables)?;
        let user_password = format!("{}:{}", kv.key.value, value);
        let authorization = base64::encode(user_password.as_bytes());

        let name = "Authorization".to_string();
        let value = format!("Basic {}", authorization);
        headers.push(http::Header { name, value });
    }

    let mut querystring: Vec<http::Param> = vec![];
    for param in request.clone().querystring_params() {
        let name = param.key.value;
        let value = eval_template(&param.value, variables)?;
        querystring.push(http::Param { name, value });
    }

    let mut form: Vec<http::Param> = vec![];
    for param in request.clone().form_params() {
        let name = param.key.value;
        let value = eval_template(&param.value, variables)?;
        form.push(http::Param { name, value });
    }
    //        if !self.clone().form_params().is_empty() {
    //            headers.push(http::ast::Header {
    //                name: String::from("Content-Type"),
    //                value: String::from("application/x-www-form-urlencoded"),
    //            });
    //        }

    let mut cookies = vec![];
    for cookie in request.clone().cookies() {
        let value = eval_template(&cookie.value, variables)?;
        let cookie = http::RequestCookie {
            name: cookie.clone().name.value,
            value,
        };
        cookies.push(cookie);
    }

    let body = match request.clone().body {
        Some(body) => eval_body(body, variables, context_dir)?,
        None => http::Body::Binary(vec![]),
    };

    let mut multipart = vec![];
    for multipart_param in request.clone().multipart_form_data() {
        let param = eval_multipart_param(multipart_param, variables, context_dir)?;
        multipart.push(param);
    }

    let content_type = if !form.is_empty() {
        Some("application/x-www-form-urlencoded".to_string())
    } else if !multipart.is_empty() {
        Some("multipart/form-data".to_string())
    } else if let Some(Body {
        value: Bytes::Json { .. },
        ..
    }) = request.body
    {
        Some("application/json".to_string())
    } else if let Some(Body {
        value: Bytes::Xml { .. },
        ..
    }) = request.body
    {
        Some("application/xml".to_string())
    } else {
        None
    };

    // add implicit content type
    //        if self.content_type().is_none() {
    //            if let Some(body) = self.body {
    //                if let Bytes::Json { .. } = body.value {
    //                    headers.push(http::ast::Header {
    //                        name: String::from("Content-Type"),
    //                        value: String::from("application/json"),
    //                    });
    //                }
    //            }
    //        }

    Ok(http::RequestSpec {
        method,
        url,
        headers,
        querystring,
        form,
        multipart,
        cookies,
        body,
        content_type,
    })
}

// pub fn get_content_type(request: Request) -> Option<Template> {
//     for header in request.headers.clone() {
//         if header.key.value.to_lowercase().as_str() == "content-type" {
//             return Some(header.value);
//         }
//     }
//     None
// }

///
/// experimental feature
/// @cookie_storage_add
///
pub fn cookie_storage_set(request: Request) -> Option<String> {
    for line_terminator in request.line_terminators.iter() {
        if let Some(s) = line_terminator.comment.clone() {
            if s.value.contains("@cookie_storage_set:") {
                let index = "#@cookie_storage_set:".to_string().len();
                let value = &s.value[index..s.value.len()].to_string().trim().to_string();
                return Some(value.to_string());
            }
        }
    }
    None
}

///
/// experimental feature
/// @cookie_storage_clear
///
pub fn cookie_storage_clear(request: Request) -> bool {
    for line_terminator in request.line_terminators.iter() {
        if let Some(s) = line_terminator.comment.clone() {
            if s.value.contains("@cookie_storage_clear") {
                return true;
            }
        }
    }
    false
}

fn eval_method(method: Method) -> http::Method {
    match method {
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

#[cfg(test)]
mod tests {
    use hurl_core::ast::SourceInfo;

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
            line_terminator0: line_terminator,
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
        let error = eval_request(hello_request(), &variables, &ContextDir::default())
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
        let http_request =
            eval_request(hello_request(), &variables, &ContextDir::default()).unwrap();
        assert_eq!(http_request, http::hello_http_request());
    }

    #[test]
    pub fn test_query_request() {
        let mut variables = HashMap::new();
        variables.insert(
            String::from("param1"),
            Value::String(String::from("value1")),
        );
        let http_request =
            eval_request(query_request(), &variables, &ContextDir::default()).unwrap();
        assert_eq!(http_request, http::query_http_request());
    }

    #[test]
    fn clear_cookie_store() {
        assert!(!cookie_storage_clear(hello_request()));

        let line_terminator = LineTerminator {
            space0: whitespace(),
            comment: None,
            newline: whitespace(),
        };
        assert!(cookie_storage_clear(Request {
            line_terminators: vec![LineTerminator {
                space0: whitespace(),
                comment: Some(Comment {
                    value: "@cookie_storage_clear".to_string()
                }),
                newline: whitespace(),
            }],
            space0: whitespace(),
            method: Method::Get,
            space1: whitespace(),
            url: Template {
                elements: vec![TemplateElement::String {
                    value: String::from("http:///localhost"),
                    encoded: String::from("http://localhost"),
                },],
                quotes: false,
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
            line_terminator0: line_terminator,
            headers: vec![],
            sections: vec![],
            body: None,
            source_info: SourceInfo::init(0, 0, 0, 0),
        }));
    }

    #[test]
    fn add_cookie_in_storage() {
        assert_eq!(None, cookie_storage_set(hello_request()));

        let line_terminator = LineTerminator {
            space0: whitespace(),
            comment: None,
            newline: whitespace(),
        };
        assert_eq!(
            Some("localhost\tFALSE\t/\tFALSE\t0\tcookie1\tvalueA".to_string()),
            cookie_storage_set(Request {
                line_terminators: vec![LineTerminator {
                    space0: whitespace(),
                    comment: Some(Comment {
                        value:
                            "@cookie_storage_set: localhost\tFALSE\t/\tFALSE\t0\tcookie1\tvalueA"
                                .to_string()
                    }),
                    newline: whitespace(),
                }],
                space0: whitespace(),
                method: Method::Get,
                space1: whitespace(),
                url: Template {
                    elements: vec![TemplateElement::String {
                        value: String::from("http:///localhost"),
                        encoded: String::from("http://localhost"),
                    },],
                    quotes: false,
                    source_info: SourceInfo::init(0, 0, 0, 0),
                },
                line_terminator0: line_terminator,
                headers: vec![],
                sections: vec![],
                body: None,
                source_info: SourceInfo::init(0, 0, 0, 0),
            })
        );
    }
}
