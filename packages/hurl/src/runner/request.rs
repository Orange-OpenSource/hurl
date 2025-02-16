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
use std::str::FromStr;

use base64::engine::general_purpose;
use base64::Engine;
use hurl_core::ast::{
    Body, Bytes, Method, MultilineString, MultilineStringKind, Request, Template,
};

use crate::http;
use crate::http::{HeaderVec, HttpError, Url, AUTHORIZATION};
use crate::runner::error::RunnerError;
use crate::runner::{body, multipart, template, RunnerErrorKind, VariableSet};
use crate::util::path::ContextDir;

/// Transforms an AST `request` to a spec request given a set of `variables`.
pub fn eval_request(
    request: &Request,
    variables: &VariableSet,
    context_dir: &ContextDir,
) -> Result<http::RequestSpec, RunnerError> {
    let method = eval_method(&request.method);
    let url = eval_url(&request.url, variables)?;

    // Headers
    let mut headers = HeaderVec::new();
    for header in &request.headers {
        let name = template::eval_template(&header.key, variables)?;
        let value = template::eval_template(&header.value, variables)?;
        let header = http::Header::new(&name, &value);
        headers.push(header);
    }

    // Basic auth
    if let Some(kv) = &request.basic_auth() {
        let name = template::eval_template(&kv.key, variables)?;
        let value = template::eval_template(&kv.value, variables)?;
        let user_password = format!("{}:{}", name, value);
        let user_password = user_password.as_bytes();
        let authorization = general_purpose::STANDARD.encode(user_password);
        let value = format!("Basic {authorization}");
        let header = http::Header::new(AUTHORIZATION, &value);
        headers.push(header);
    }

    // Query string params
    let mut querystring = vec![];
    for param in request.querystring_params() {
        let name = template::eval_template(&param.key, variables)?;
        let value = template::eval_template(&param.value, variables)?;
        let param = http::Param { name, value };
        querystring.push(param);
    }

    // Form params
    let mut form = vec![];
    for param in request.form_params() {
        let name = template::eval_template(&param.key, variables)?;
        let value = template::eval_template(&param.value, variables)?;
        let param = http::Param { name, value };
        form.push(param);
    }

    // Cookies
    let mut cookies = vec![];
    for cookie in request.cookies() {
        let name = template::eval_template(&cookie.name, variables)?;
        let value = template::eval_template(&cookie.value, variables)?;
        let cookie = http::RequestCookie { name, value };
        cookies.push(cookie);
    }

    let body = match &request.body {
        Some(body) => body::eval_body(body, variables, context_dir)?,
        None => http::Body::Binary(vec![]),
    };

    let mut multipart = vec![];
    for multipart_param in request.multipart_form_data() {
        let param = multipart::eval_multipart_param(multipart_param, variables, context_dir)?;
        multipart.push(param);
    }

    let implicit_content_type = if !form.is_empty() {
        Some("application/x-www-form-urlencoded".to_string())
    } else if !multipart.is_empty() {
        Some("multipart/form-data".to_string())
    } else if let Some(Body {
        value:
            Bytes::Json { .. }
            | Bytes::MultilineString(MultilineString {
                kind: MultilineStringKind::GraphQl(..),
                ..
            })
            | Bytes::MultilineString(MultilineString {
                kind: MultilineStringKind::Json(..),
                ..
            }),
        ..
    }) = request.body
    {
        Some("application/json".to_string())
    } else if let Some(Body {
        value:
            Bytes::Xml { .. }
            | Bytes::MultilineString(MultilineString {
                kind: MultilineStringKind::Xml(..),
                ..
            }),
        ..
    }) = request.body
    {
        Some("application/xml".to_string())
    } else {
        None
    };

    Ok(http::RequestSpec {
        method,
        url,
        headers,
        querystring,
        form,
        multipart,
        cookies,
        body,
        implicit_content_type,
    })
}

fn eval_url(url_template: &Template, variables: &VariableSet) -> Result<Url, RunnerError> {
    let url = template::eval_template(url_template, variables)?;
    Url::from_str(&url).map_err(|e| {
        let source_info = url_template.source_info;
        let message = if let HttpError::InvalidUrl(_, message) = e {
            message
        } else {
            String::new()
        };
        let runner_error_kind = RunnerErrorKind::InvalidUrl { url, message };
        RunnerError::new(source_info, runner_error_kind, false)
    })
}

/// Experimental feature
/// @cookie_storage_add
pub fn cookie_storage_set(request: &Request) -> Option<String> {
    for line_terminator in request.line_terminators.iter() {
        if let Some(s) = &line_terminator.comment {
            if s.value.contains("@cookie_storage_set:") {
                let index = "#@cookie_storage_set:".to_string().len();
                let value = &s.value[index..s.value.len()].to_string().trim().to_string();
                return Some(value.to_string());
            }
        }
    }
    None
}

/// Experimental feature
/// @cookie_storage_clear
pub fn cookie_storage_clear(request: &Request) -> bool {
    for line_terminator in request.line_terminators.iter() {
        if let Some(s) = &line_terminator.comment {
            if s.value.contains("@cookie_storage_clear") {
                return true;
            }
        }
    }
    false
}

fn eval_method(method: &Method) -> http::Method {
    http::Method(method.0.clone())
}

#[cfg(test)]
mod tests {
    use hurl_core::ast::{
        Comment, Expr, ExprKind, KeyValue, LineTerminator, Placeholder, Section, SectionValue,
        SourceInfo, TemplateElement, Variable, Whitespace,
    };
    use hurl_core::reader::Pos;
    use hurl_core::typing::ToSource;

    use super::super::error::RunnerErrorKind;
    use super::*;
    use crate::runner::Value;

    fn whitespace() -> Whitespace {
        Whitespace {
            value: String::from(" "),
            source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
        }
    }

    fn hello_request() -> Request {
        let line_terminator = LineTerminator {
            space0: whitespace(),
            comment: None,
            newline: whitespace(),
        };
        Request {
            line_terminators: vec![],
            space0: whitespace(),
            method: Method("GET".to_string()),
            space1: whitespace(),
            url: Template {
                elements: vec![
                    TemplateElement::Placeholder(Placeholder {
                        space0: whitespace(),
                        expr: Expr {
                            kind: ExprKind::Variable(Variable {
                                name: "base_url".to_string(),
                                source_info: SourceInfo::new(Pos::new(1, 7), Pos::new(1, 15)),
                            }),
                            source_info: SourceInfo::new(Pos::new(1, 7), Pos::new(1, 15)),
                        },
                        space1: whitespace(),
                    }),
                    TemplateElement::String {
                        value: "/hello".to_string(),
                        source: "/hello".to_source(),
                    },
                ],
                delimiter: None,
                source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
            },
            line_terminator0: line_terminator,
            headers: vec![],
            sections: vec![],
            body: None,
            source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
        }
    }

    fn simple_key_value(key: Template, value: Template) -> KeyValue {
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

    fn query_request() -> Request {
        let line_terminator = LineTerminator {
            space0: whitespace(),
            comment: None,
            newline: whitespace(),
        };
        Request {
            line_terminators: vec![],
            space0: whitespace(),
            method: Method("GET".to_string()),
            space1: whitespace(),
            url: Template {
                elements: vec![TemplateElement::String {
                    value: "http://localhost:8000/querystring-params".to_string(),
                    source: "http://localhost:8000/querystring-params".to_source(),
                }],
                delimiter: None,
                source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
            },
            line_terminator0: line_terminator.clone(),
            headers: vec![],
            sections: vec![Section {
                line_terminators: vec![],
                space0: whitespace(),
                line_terminator0: line_terminator,
                value: SectionValue::QueryParams(
                    vec![
                        simple_key_value(
                            Template {
                                delimiter: None,
                                elements: vec![TemplateElement::String {
                                    value: "param1".to_string(),
                                    source: "param1".to_source(),
                                }],
                                source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                            },
                            Template {
                                delimiter: None,
                                elements: vec![TemplateElement::Placeholder(Placeholder {
                                    space0: whitespace(),
                                    expr: Expr {
                                        kind: ExprKind::Variable(Variable {
                                            name: "param1".to_string(),
                                            source_info: SourceInfo::new(
                                                Pos::new(1, 7),
                                                Pos::new(1, 15),
                                            ),
                                        }),
                                        source_info: SourceInfo::new(
                                            Pos::new(1, 7),
                                            Pos::new(1, 15),
                                        ),
                                    },
                                    space1: whitespace(),
                                })],
                                source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                            },
                        ),
                        simple_key_value(
                            Template {
                                delimiter: None,
                                elements: vec![TemplateElement::String {
                                    value: "param2".to_string(),
                                    source: "param2".to_source(),
                                }],
                                source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                            },
                            Template {
                                delimiter: None,
                                elements: vec![TemplateElement::String {
                                    value: "a b".to_string(),
                                    source: "a b".to_source(),
                                }],
                                source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                            },
                        ),
                    ],
                    false,
                ),
                source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
            }],
            body: None,
            source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
        }
    }

    #[test]
    fn test_error_variable() {
        let variables = VariableSet::new();
        let error = eval_request(&hello_request(), &variables, &ContextDir::default())
            .err()
            .unwrap();
        assert_eq!(
            error.source_info,
            SourceInfo::new(Pos::new(1, 7), Pos::new(1, 15))
        );
        assert_eq!(
            error.kind,
            RunnerErrorKind::TemplateVariableNotDefined {
                name: String::from("base_url")
            }
        );
    }

    #[test]
    fn test_hello_request() {
        let mut variables = VariableSet::new();
        variables
            .insert(
                String::from("base_url"),
                Value::String(String::from("http://localhost:8000")),
            )
            .unwrap();
        let http_request =
            eval_request(&hello_request(), &variables, &ContextDir::default()).unwrap();
        assert_eq!(http_request, http::hello_http_request());
    }

    #[test]
    fn test_query_request() {
        let mut variables = VariableSet::new();
        variables
            .insert(
                String::from("param1"),
                Value::String(String::from("value1")),
            )
            .unwrap();
        let http_request =
            eval_request(&query_request(), &variables, &ContextDir::default()).unwrap();
        assert_eq!(http_request, http::query_http_request());
    }

    #[test]
    fn clear_cookie_store() {
        assert!(!cookie_storage_clear(&hello_request()));

        let line_terminator = LineTerminator {
            space0: whitespace(),
            comment: None,
            newline: whitespace(),
        };
        assert!(cookie_storage_clear(&Request {
            line_terminators: vec![LineTerminator {
                space0: whitespace(),
                comment: Some(Comment {
                    value: "@cookie_storage_clear".to_string(),
                    source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                }),
                newline: whitespace(),
            }],
            space0: whitespace(),
            method: Method("GET".to_string()),
            space1: whitespace(),
            url: Template {
                elements: vec![TemplateElement::String {
                    value: "http:///localhost".to_string(),
                    source: "http://localhost".to_source(),
                },],
                delimiter: None,
                source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
            },
            line_terminator0: line_terminator,
            headers: vec![],
            sections: vec![],
            body: None,
            source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
        }));
    }

    #[test]
    fn add_cookie_in_storage() {
        assert_eq!(None, cookie_storage_set(&hello_request()));

        let line_terminator = LineTerminator {
            space0: whitespace(),
            comment: None,
            newline: whitespace(),
        };
        assert_eq!(
            Some("localhost\tFALSE\t/\tFALSE\t0\tcookie1\tvalueA".to_string()),
            cookie_storage_set(&Request {
                line_terminators: vec![LineTerminator {
                    space0: whitespace(),
                    comment: Some(Comment {
                        value:
                            "@cookie_storage_set: localhost\tFALSE\t/\tFALSE\t0\tcookie1\tvalueA"
                                .to_string(),
                        source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                    }),
                    newline: whitespace(),
                }],
                space0: whitespace(),
                method: Method("GET".to_string()),
                space1: whitespace(),
                url: Template {
                    elements: vec![TemplateElement::String {
                        value: "http:///localhost".to_string(),
                        source: "http://localhost".to_source(),
                    },],
                    delimiter: None,
                    source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                },
                line_terminator0: line_terminator,
                headers: vec![],
                sections: vec![],
                body: None,
                source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
            })
        );
    }
}
