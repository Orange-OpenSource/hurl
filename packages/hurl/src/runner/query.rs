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

use regex::Regex;
use std::collections::HashMap;

use super::core::{Error, RunnerError};
use super::template::eval_template;
use super::value::Value;
use super::xpath;
use crate::http;
use crate::jsonpath;
use crate::runner::subquery::eval_subquery;
use hurl_core::ast::*;
use sha2::Digest;

pub type QueryResult = Result<Option<Value>, Error>;

pub fn eval_query(
    query: Query,
    variables: &HashMap<String, Value>,
    http_response: http::Response,
) -> QueryResult {
    let value = eval_query_value(query.clone(), variables, http_response)?;
    if let Some((_, subquery)) = query.subquery {
        if let Some(value) = value {
            eval_subquery(subquery, value, variables)
        } else {
            Err(Error {
                source_info: subquery.source_info,
                inner: RunnerError::SubqueryInvalidInput("none".to_string()),
                assert: false,
            })
        }
    } else {
        Ok(value)
    }
}
pub fn eval_query_value(
    query: Query,
    variables: &HashMap<String, Value>,
    http_response: http::Response,
) -> QueryResult {
    match query.value.clone() {
        QueryValue::Status {} => Ok(Some(Value::Integer(i64::from(http_response.status)))),
        QueryValue::Header { name, .. } => {
            let header_name = eval_template(&name, variables)?;
            let values = http_response.get_header_values(&header_name);
            if values.is_empty() {
                Ok(None)
            } else if values.len() == 1 {
                let value = values.first().unwrap().to_string();
                Ok(Some(Value::String(value)))
            } else {
                let values = values
                    .iter()
                    .map(|v| Value::String(v.to_string()))
                    .collect();
                Ok(Some(Value::List(values)))
            }
        }
        QueryValue::Cookie {
            expr: CookiePath { name, attribute },
            ..
        } => {
            let cookie_name = eval_template(&name, variables)?;
            match http_response.get_cookie(cookie_name) {
                None => Ok(None),
                Some(cookie) => {
                    let attribute_name = if let Some(attribute) = attribute {
                        attribute.name
                    } else {
                        CookieAttributeName::Value("Value".to_string())
                    };
                    Ok(eval_cookie_attribute_name(attribute_name, cookie))
                }
            }
        }
        QueryValue::Body {} => {
            // can return a string if encoding is known and utf8
            match http_response.text() {
                Ok(s) => Ok(Some(Value::String(s))),
                Err(inner) => Err(Error {
                    source_info: query.source_info,
                    inner: RunnerError::from(inner),
                    assert: false,
                }),
            }
        }
        QueryValue::Xpath { expr, .. } => {
            let source_info = expr.source_info.clone();
            let value = eval_template(&expr, variables)?;
            match http_response.text() {
                Err(inner) => Err(Error {
                    source_info: query.source_info,
                    inner: RunnerError::from(inner),
                    assert: false,
                }),
                Ok(xml) => {
                    let result = if http_response.is_html() {
                        xpath::eval_html(&xml, &value)
                    } else {
                        xpath::eval_xml(&xml, &value)
                    };
                    match result {
                        Ok(value) => Ok(Some(value)),
                        Err(xpath::XpathError::InvalidXml {}) => Err(Error {
                            source_info: query.source_info,
                            inner: RunnerError::QueryInvalidXml,
                            assert: false,
                        }),
                        Err(xpath::XpathError::InvalidHtml {}) => Err(Error {
                            source_info: query.source_info,
                            inner: RunnerError::QueryInvalidXml,
                            assert: false,
                        }),
                        Err(xpath::XpathError::Eval {}) => Err(Error {
                            source_info,
                            inner: RunnerError::QueryInvalidXpathEval,
                            assert: false,
                        }),
                        Err(xpath::XpathError::Unsupported {}) => {
                            panic!("Unsupported xpath {}", value); // good usecase for panic - I could nmot reporduce this usecase myself
                        }
                    }
                }
            }
        }
        QueryValue::Jsonpath { expr, .. } => {
            let value = eval_template(&expr, variables)?;
            let source_info = expr.source_info;
            let jsonpath_query = match jsonpath::parse(value.as_str()) {
                Ok(q) => q,
                Err(_) => {
                    return Err(Error {
                        source_info,
                        inner: RunnerError::QueryInvalidJsonpathExpression { value },
                        assert: false,
                    });
                }
            };
            let json = match http_response.text() {
                Err(inner) => {
                    return Err(Error {
                        source_info: query.source_info,
                        inner: RunnerError::from(inner),
                        assert: false,
                    });
                }
                Ok(v) => v,
            };
            let value = match serde_json::from_str(json.as_str()) {
                Err(_) => {
                    return Err(Error {
                        source_info: query.source_info,
                        inner: RunnerError::QueryInvalidJson,
                        assert: false,
                    });
                }
                Ok(v) => v,
            };
            let results = jsonpath_query.eval(value);
            if results.is_empty() {
                Ok(None)
            } else if results.len() == 1 {
                // list coercions
                Ok(Some(Value::from_json(results.get(0).unwrap())))
            } else {
                Ok(Some(Value::from_json(&serde_json::Value::Array(results))))
            }
        }
        QueryValue::Regex { value, .. } => {
            let s = match http_response.text() {
                Err(inner) => {
                    return Err(Error {
                        source_info: query.source_info,
                        inner: RunnerError::from(inner),
                        assert: false,
                    });
                }
                Ok(v) => v,
            };
            let re = match value {
                RegexValue::Template(t) => {
                    let value = eval_template(&t, variables)?;
                    match Regex::new(value.as_str()) {
                        Ok(re) => re,
                        Err(_) => {
                            let source_info = t.source_info;
                            return Err(Error {
                                source_info,
                                inner: RunnerError::InvalidRegex(),
                                assert: false,
                            });
                        }
                    }
                }
                RegexValue::Regex(re) => re.inner,
            };
            match re.captures(s.as_str()) {
                Some(captures) => match captures.get(1) {
                    Some(v) => Ok(Some(Value::String(v.as_str().to_string()))),
                    None => Ok(None),
                },
                None => Ok(None),
            }
        }
        QueryValue::Variable { name, .. } => {
            let name = eval_template(&name, variables)?;
            if let Some(value) = variables.get(name.as_str()) {
                Ok(Some(value.clone()))
            } else {
                Ok(None)
            }
        }
        QueryValue::Duration {} => Ok(Some(Value::Integer(
            http_response.duration.as_millis() as i64
        ))),
        QueryValue::Bytes {} => match http_response.uncompress_body() {
            Ok(s) => Ok(Some(Value::Bytes(s))),
            Err(inner) => Err(Error {
                source_info: query.source_info,
                inner: RunnerError::from(inner),
                assert: false,
            }),
        },
        QueryValue::Sha256 {} => {
            let bytes = match http_response.uncompress_body() {
                Ok(s) => s,
                Err(inner) => {
                    return Err(Error {
                        source_info: query.source_info,
                        inner: RunnerError::from(inner),
                        assert: false,
                    })
                }
            };
            let mut hasher = sha2::Sha256::new();
            hasher.update(bytes);
            let result = hasher.finalize();
            let bytes = Value::Bytes(result[..].to_vec());
            Ok(Some(bytes))
        }
        QueryValue::Md5 {} => {
            let bytes = match http_response.uncompress_body() {
                Ok(s) => s,
                Err(inner) => {
                    return Err(Error {
                        source_info: query.source_info,
                        inner: RunnerError::from(inner),
                        assert: false,
                    })
                }
            };
            let bytes = md5::compute(bytes).to_vec();
            Ok(Some(Value::Bytes(bytes)))
        }
    }
}

pub fn eval_cookie_attribute_name(
    cookie_attribute_name: CookieAttributeName,
    cookie: http::ResponseCookie,
) -> Option<Value> {
    match cookie_attribute_name {
        CookieAttributeName::Value(_) => Some(Value::String(cookie.value)),
        CookieAttributeName::Expires(_) => cookie.expires().map(Value::String),
        CookieAttributeName::MaxAge(_) => cookie.max_age().map(Value::Integer),
        CookieAttributeName::Domain(_) => cookie.domain().map(Value::String),
        CookieAttributeName::Path(_) => cookie.path().map(Value::String),
        CookieAttributeName::Secure(_) => {
            if cookie.has_secure() {
                Some(Value::Unit)
            } else {
                None
            }
        }
        CookieAttributeName::HttpOnly(_) => {
            if cookie.has_httponly() {
                Some(Value::Unit)
            } else {
                None
            }
        }
        CookieAttributeName::SameSite(_) => cookie.samesite().map(Value::String),
    }
}

impl Value {
    pub fn from_json(value: &serde_json::Value) -> Value {
        match value {
            serde_json::Value::Null => Value::Null,
            serde_json::Value::Bool(bool) => Value::Bool(*bool),
            serde_json::Value::Number(n) => {
                if n.is_f64() {
                    Value::from_f64(n.as_f64().unwrap())
                } else {
                    Value::Integer(n.as_i64().unwrap())
                }
            }
            serde_json::Value::String(s) => Value::String(s.to_string()),
            serde_json::Value::Array(elements) => {
                Value::List(elements.iter().map(Value::from_json).collect())
            }
            serde_json::Value::Object(map) => {
                let mut elements = vec![];
                for (key, value) in map {
                    elements.push((key.to_string(), Value::from_json(value)));
                    //
                }
                Value::Object(elements)
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use hex_literal::hex;
    use hurl_core::ast::{Pos, SourceInfo};

    pub fn xpath_invalid_query() -> Query {
        // xpath ???
        let whitespace = Whitespace {
            value: String::from(" "),
            source_info: SourceInfo::init(0, 0, 0, 0),
        };
        Query {
            source_info: SourceInfo::init(1, 1, 1, 13),
            value: QueryValue::Xpath {
                space0: whitespace,
                expr: Template {
                    quotes: true,
                    elements: vec![TemplateElement::String {
                        value: "???".to_string(),
                        encoded: "???".to_string(),
                    }],
                    source_info: SourceInfo::init(1, 7, 1, 10),
                },
            },
            subquery: None,
        }
    }

    pub fn xpath_count_user_query() -> Query {
        let whitespace = Whitespace {
            value: String::from(" "),
            source_info: SourceInfo::init(0, 0, 0, 0),
        };
        Query {
            source_info: SourceInfo::init(1, 1, 1, 13),
            value: QueryValue::Xpath {
                space0: whitespace,
                expr: Template {
                    quotes: true,
                    elements: vec![TemplateElement::String {
                        value: "count(//user)".to_string(),
                        encoded: "count(//user)".to_string(),
                    }],
                    source_info: SourceInfo::init(0, 0, 0, 0),
                },
            },
            subquery: None,
        }
    }

    pub fn xpath_users() -> Query {
        let whitespace = Whitespace {
            value: String::from(" "),
            source_info: SourceInfo::init(0, 0, 0, 0),
        };
        Query {
            source_info: SourceInfo::init(1, 1, 1, 13),
            value: QueryValue::Xpath {
                space0: whitespace,
                expr: Template {
                    quotes: true,
                    elements: vec![TemplateElement::String {
                        value: "//user".to_string(),
                        encoded: "/user".to_string(),
                    }],
                    source_info: SourceInfo::init(0, 0, 0, 0),
                },
            },
            subquery: None,
        }
    }

    pub fn json_http_response() -> http::Response {
        http::Response {
            version: http::Version::Http10,
            status: 0,
            headers: vec![],
            body: String::into_bytes(
                r#"
{
  "success":false,
  "errors": [
    { "id": "error1"},
    {"id": "error2"}
  ]
}
"#
                .to_string(),
            ),
            duration: Default::default(),
        }
    }

    pub fn jsonpath_success() -> Query {
        // jsonpath $.success
        Query {
            source_info: SourceInfo::init(1, 1, 1, 19),
            value: QueryValue::Jsonpath {
                space0: Whitespace {
                    value: String::from(""),
                    source_info: SourceInfo::init(1, 9, 1, 10),
                },
                expr: Template {
                    elements: vec![TemplateElement::String {
                        value: String::from("$.success"),
                        encoded: String::from("$.success"),
                    }],
                    quotes: true,
                    //delimiter: "".to_string(),
                    source_info: SourceInfo::init(1, 10, 1, 19),
                },
            },
            subquery: None,
        }
    }

    pub fn jsonpath_errors() -> Query {
        // jsonpath $.errors
        Query {
            source_info: SourceInfo::init(1, 1, 1, 19),
            value: QueryValue::Jsonpath {
                space0: Whitespace {
                    value: String::from(""),
                    source_info: SourceInfo::init(1, 9, 1, 10),
                },
                expr: Template {
                    elements: vec![TemplateElement::String {
                        value: String::from("$.errors"),
                        encoded: String::from("$.errors"),
                    }],
                    quotes: true,
                    //   delimiter: "".to_string(),
                    source_info: SourceInfo::init(1, 10, 1, 18),
                },
            },
            subquery: None,
        }
    }

    pub fn jsonpath_errors_count() -> Query {
        // jsonpath "$.errors" count
        Query {
            source_info: SourceInfo::init(1, 1, 1, 19),
            value: QueryValue::Jsonpath {
                space0: Whitespace {
                    value: String::from(""),
                    source_info: SourceInfo::init(1, 9, 1, 10),
                },
                expr: Template {
                    elements: vec![TemplateElement::String {
                        value: String::from("$.errors"),
                        encoded: String::from("$.errors"),
                    }],
                    quotes: true,
                    //   delimiter: "".to_string(),
                    source_info: SourceInfo::init(1, 10, 1, 18),
                },
            },
            subquery: Some((
                Whitespace {
                    value: "".to_string(),
                    source_info: SourceInfo::init(0, 0, 0, 0),
                },
                Subquery {
                    source_info: SourceInfo::init(0, 0, 0, 0),
                    value: SubqueryValue::Count {},
                },
            )),
        }
    }

    pub fn jsonpath_duration() -> Query {
        // jsonpath $.errors
        Query {
            source_info: SourceInfo::init(1, 1, 1, 19),
            value: QueryValue::Jsonpath {
                space0: Whitespace {
                    value: String::from(""),
                    source_info: SourceInfo::init(1, 9, 1, 10),
                },
                expr: Template {
                    elements: vec![TemplateElement::String {
                        value: String::from("$.duration"),
                        encoded: String::from("$.duration"),
                    }],
                    quotes: true,
                    //   delimiter: "".to_string(),
                    source_info: SourceInfo::init(1, 10, 1, 18),
                },
            },
            subquery: None,
        }
    }

    pub fn regex_name() -> Query {
        // regex "Hello ([a-zA-Z]+)!"
        Query {
            source_info: SourceInfo::init(1, 1, 1, 26),
            value: QueryValue::Regex {
                space0: Whitespace {
                    value: String::from(""),
                    source_info: SourceInfo::init(1, 6, 1, 7),
                },
                value: RegexValue::Template(Template {
                    quotes: true,
                    elements: vec![TemplateElement::String {
                        value: "Hello ([a-zA-Z]+)!".to_string(),
                        encoded: "Hello ([a-zA-Z]+)!".to_string(),
                    }],
                    source_info: SourceInfo::init(1, 7, 1, 26),
                }),
            },
            subquery: None,
        }
    }

    pub fn regex_invalid() -> Query {
        // regex ???"
        Query {
            source_info: SourceInfo::init(1, 1, 1, 26),
            value: QueryValue::Regex {
                space0: Whitespace {
                    value: String::from(""),
                    source_info: SourceInfo::init(1, 6, 1, 7),
                },
                value: RegexValue::Template(Template {
                    quotes: true,
                    elements: vec![TemplateElement::String {
                        value: "???".to_string(),
                        encoded: "???".to_string(),
                    }],
                    source_info: SourceInfo::init(1, 7, 1, 10),
                }),
            },
            subquery: None,
        }
    }

    #[test]
    fn test_query_status() {
        let variables = HashMap::new();
        assert_eq!(
            eval_query(
                Query {
                    source_info: SourceInfo::init(0, 0, 0, 0),
                    value: QueryValue::Status {},
                    subquery: None
                },
                &variables,
                http::hello_http_response(),
            )
            .unwrap()
            .unwrap(),
            Value::Integer(200)
        );
    }

    #[test]
    fn test_header_not_found() {
        let variables = HashMap::new();
        // header Custom
        let query_header = Query {
            source_info: SourceInfo::init(0, 0, 0, 0),
            value: QueryValue::Header {
                space0: Whitespace {
                    value: String::from(" "),
                    source_info: SourceInfo::init(1, 7, 1, 8),
                },
                name: Template {
                    quotes: true,
                    elements: vec![TemplateElement::String {
                        value: "Custom".to_string(),
                        encoded: "Custom".to_string(),
                    }],
                    source_info: SourceInfo::init(2, 8, 2, 14),
                },
            },
            subquery: None,
        };
        //    let error = query_header.eval(http::hello_http_response()).err().unwrap();
        //    assert_eq!(error.source_info.start, Pos { line: 1, column: 8 });
        //    assert_eq!(error.inner, RunnerError::QueryHeaderNotFound);
        assert_eq!(
            eval_query(query_header, &variables, http::hello_http_response()).unwrap(),
            None
        );
    }

    #[test]
    fn test_header() {
        // header Content-Type
        let variables = HashMap::new();
        let query_header = Query {
            source_info: SourceInfo::init(0, 0, 0, 0),
            value: QueryValue::Header {
                space0: Whitespace {
                    value: String::from(" "),
                    source_info: SourceInfo::init(1, 7, 1, 8),
                },
                name: Template {
                    quotes: true,
                    elements: vec![TemplateElement::String {
                        value: "Content-Type".to_string(),
                        encoded: "Content-Type".to_string(),
                    }],
                    source_info: SourceInfo::init(1, 8, 1, 16),
                },
            },
            subquery: None,
        };
        assert_eq!(
            eval_query(query_header, &variables, http::hello_http_response())
                .unwrap()
                .unwrap(),
            Value::String(String::from("text/html; charset=utf-8"))
        );
    }

    #[test]
    fn test_query_cookie() {
        let variables = HashMap::new();
        let space = Whitespace {
            value: String::from(""),
            source_info: SourceInfo::init(0, 0, 0, 0),
        };
        let response = http::Response {
            version: http::Version::Http10,
            status: 0,
            headers: vec![
                http::Header {
                    name: "Set-Cookie".to_string(),
                    value: "LSID=DQAAAKEaem_vYg; Path=/accounts; Expires=Wed, 13 Jan 2021 22:23:01 GMT; Secure; HttpOnly".to_string(),
                }
            ],
            body: vec![],
            duration: Default::default(),
        };

        // cookie "LSID"
        let query = Query {
            source_info: SourceInfo::init(0, 0, 0, 0),
            value: QueryValue::Cookie {
                space0: space.clone(),
                expr: CookiePath {
                    name: Template {
                        quotes: true,
                        elements: vec![TemplateElement::String {
                            value: "LSID".to_string(),
                            encoded: "LSID".to_string(),
                        }],
                        source_info: SourceInfo::init(0, 0, 0, 0),
                    },
                    attribute: None,
                },
            },
            subquery: None,
        };
        assert_eq!(
            eval_query(query, &variables, response.clone())
                .unwrap()
                .unwrap(),
            Value::String("DQAAAKEaem_vYg".to_string())
        );

        // cookie "LSID[Path]"
        let query = Query {
            source_info: SourceInfo::init(0, 0, 0, 0),
            value: QueryValue::Cookie {
                space0: space.clone(),
                expr: CookiePath {
                    name: Template {
                        quotes: true,
                        elements: vec![TemplateElement::String {
                            value: "LSID".to_string(),
                            encoded: "LSID".to_string(),
                        }],
                        source_info: SourceInfo::init(0, 0, 0, 0),
                    },
                    attribute: Some(CookieAttribute {
                        space0: space.clone(),
                        name: CookieAttributeName::Path("Path".to_string()),
                        space1: space.clone(),
                    }),
                },
            },
            subquery: None,
        };
        assert_eq!(
            eval_query(query, &variables, response.clone())
                .unwrap()
                .unwrap(),
            Value::String("/accounts".to_string())
        );

        // cookie "LSID[Secure]"
        let query = Query {
            source_info: SourceInfo::init(0, 0, 0, 0),
            value: QueryValue::Cookie {
                space0: space.clone(),
                expr: CookiePath {
                    name: Template {
                        quotes: true,
                        elements: vec![TemplateElement::String {
                            value: "LSID".to_string(),
                            encoded: "LSID".to_string(),
                        }],
                        source_info: SourceInfo::init(0, 0, 0, 0),
                    },
                    attribute: Some(CookieAttribute {
                        space0: space.clone(),
                        name: CookieAttributeName::Secure("Secure".to_string()),
                        space1: space.clone(),
                    }),
                },
            },
            subquery: None,
        };
        assert_eq!(
            eval_query(query, &variables, response.clone())
                .unwrap()
                .unwrap(),
            Value::Unit
        );

        // cookie "LSID[Domain]"
        let query = Query {
            source_info: SourceInfo::init(0, 0, 0, 0),
            value: QueryValue::Cookie {
                space0: space.clone(),
                expr: CookiePath {
                    name: Template {
                        quotes: true,
                        elements: vec![TemplateElement::String {
                            value: "LSID".to_string(),
                            encoded: "LSID".to_string(),
                        }],
                        source_info: SourceInfo::init(0, 0, 0, 0),
                    },
                    attribute: Some(CookieAttribute {
                        space0: space.clone(),
                        name: CookieAttributeName::Domain("Domain".to_string()),
                        space1: space,
                    }),
                },
            },
            subquery: None,
        };
        assert_eq!(eval_query(query, &variables, response).unwrap(), None);
    }

    #[test]
    fn test_eval_cookie_attribute_name() {
        let cookie = http::ResponseCookie {
            name: "LSID".to_string(),
            value: "DQAAAKEaem_vYg".to_string(),
            attributes: vec![
                http::CookieAttribute {
                    name: "Path".to_string(),
                    value: Some("/accounts".to_string()),
                },
                http::CookieAttribute {
                    name: "Expires".to_string(),
                    value: Some("Wed, 13 Jan 2021 22:23:01 GMT".to_string()),
                },
                http::CookieAttribute {
                    name: "Secure".to_string(),
                    value: None,
                },
                http::CookieAttribute {
                    name: "HttpOnly".to_string(),
                    value: None,
                },
            ],
        };
        assert_eq!(
            eval_cookie_attribute_name(CookieAttributeName::Value("_".to_string()), cookie.clone())
                .unwrap(),
            Value::String("DQAAAKEaem_vYg".to_string())
        );
        assert_eq!(
            eval_cookie_attribute_name(
                CookieAttributeName::Domain("_".to_string()),
                cookie.clone(),
            ),
            None
        );
        assert_eq!(
            eval_cookie_attribute_name(CookieAttributeName::Path("_".to_string()), cookie.clone())
                .unwrap(),
            Value::String("/accounts".to_string())
        );
        assert_eq!(
            eval_cookie_attribute_name(
                CookieAttributeName::MaxAge("_".to_string()),
                cookie.clone(),
            ),
            None
        );
        assert_eq!(
            eval_cookie_attribute_name(
                CookieAttributeName::Expires("_".to_string()),
                cookie.clone(),
            )
            .unwrap(),
            Value::String("Wed, 13 Jan 2021 22:23:01 GMT".to_string())
        );
        assert_eq!(
            eval_cookie_attribute_name(
                CookieAttributeName::Secure("_".to_string()),
                cookie.clone(),
            )
            .unwrap(),
            Value::Unit
        );
        assert_eq!(
            eval_cookie_attribute_name(
                CookieAttributeName::HttpOnly("_".to_string()),
                cookie.clone(),
            )
            .unwrap(),
            Value::Unit
        );
        assert_eq!(
            eval_cookie_attribute_name(CookieAttributeName::SameSite("_".to_string()), cookie),
            None
        );
    }

    #[test]
    fn test_body() {
        let variables = HashMap::new();
        assert_eq!(
            eval_query(
                Query {
                    source_info: SourceInfo::init(0, 0, 0, 0),
                    value: QueryValue::Body {},
                    subquery: None
                },
                &variables,
                http::hello_http_response(),
            )
            .unwrap()
            .unwrap(),
            Value::String(String::from("Hello World!"))
        );
        let error = eval_query(
            Query {
                source_info: SourceInfo::init(1, 1, 1, 2),
                value: QueryValue::Body {},
                subquery: None,
            },
            &variables,
            http::bytes_http_response(),
        )
        .err()
        .unwrap();
        assert_eq!(error.source_info, SourceInfo::init(1, 1, 1, 2));
        assert_eq!(
            error.inner,
            RunnerError::InvalidDecoding {
                charset: "utf-8".to_string()
            }
        );
    }

    #[test]
    fn test_query_invalid_utf8() {
        let variables = HashMap::new();
        let http_response = http::Response {
            version: http::Version::Http10,
            status: 0,
            headers: vec![],
            body: vec![200],
            duration: Default::default(),
        };
        let error = eval_query(xpath_users(), &variables, http_response)
            .err()
            .unwrap();
        assert_eq!(error.source_info.start, Pos { line: 1, column: 1 });
        assert_eq!(
            error.inner,
            RunnerError::InvalidDecoding {
                charset: "utf-8".to_string()
            }
        );
    }

    #[test]
    fn test_query_xpath_error_eval() {
        let variables = HashMap::new();
        // xpath ^^^
        let query = Query {
            source_info: SourceInfo::init(0, 0, 0, 0),
            value: QueryValue::Xpath {
                space0: Whitespace {
                    value: String::from(" "),
                    source_info: SourceInfo::init(1, 6, 1, 7),
                },
                expr: Template {
                    quotes: true,
                    elements: vec![TemplateElement::String {
                        value: "^^^".to_string(),
                        encoded: "^^^".to_string(),
                    }],
                    source_info: SourceInfo::init(1, 7, 1, 10),
                },
            },
            subquery: None,
        };
        let error = eval_query(query, &variables, http::xml_two_users_http_response())
            .err()
            .unwrap();
        assert_eq!(error.inner, RunnerError::QueryInvalidXpathEval);
        assert_eq!(error.source_info.start, Pos { line: 1, column: 7 });
    }

    #[test]
    fn test_query_xpath() {
        let variables = HashMap::new();

        assert_eq!(
            eval_query(
                xpath_users(),
                &variables,
                http::xml_two_users_http_response(),
            )
            .unwrap()
            .unwrap(),
            Value::Nodeset(2)
        );
        assert_eq!(
            eval_query(
                xpath_count_user_query(),
                &variables,
                http::xml_two_users_http_response(),
            )
            .unwrap()
            .unwrap(),
            Value::Float(2.0)
        );
    }

    #[cfg(test)]
    pub fn xpath_html_charset() -> Query {
        // $x("normalize-space(/html/head/meta/@charset)")
        let whitespace = Whitespace {
            value: String::from(" "),
            source_info: SourceInfo::init(0, 0, 0, 0),
        };
        Query {
            source_info: SourceInfo::init(1, 1, 1, 13),
            value: QueryValue::Xpath {
                space0: whitespace,
                expr: Template {
                    quotes: true,
                    elements: vec![TemplateElement::String {
                        value: "normalize-space(/html/head/meta/@charset)".to_string(),
                        encoded: "normalize-space(/html/head/meta/@charset)".to_string(),
                    }],
                    source_info: SourceInfo::init(0, 0, 0, 0),
                },
            },
            subquery: None,
        }
    }

    #[test]
    fn test_query_xpath_with_html() {
        let variables = HashMap::new();
        assert_eq!(
            eval_query(xpath_html_charset(), &variables, http::html_http_response())
                .unwrap()
                .unwrap(),
            Value::String(String::from("UTF-8"))
        );
    }

    #[test]
    fn test_query_jsonpath_invalid_expression() {
        let variables = HashMap::new();

        // jsonpath xxx
        let jsonpath_query = Query {
            source_info: SourceInfo::init(0, 0, 0, 0),
            value: QueryValue::Jsonpath {
                space0: Whitespace {
                    value: String::from(""),
                    source_info: SourceInfo::init(1, 9, 1, 10),
                },
                expr: Template {
                    elements: vec![TemplateElement::String {
                        value: String::from("xxx"),
                        encoded: String::from("xxx"),
                    }],
                    quotes: true,
                    // delimiter: "".to_string(),
                    source_info: SourceInfo::init(1, 10, 1, 13),
                },
            },
            subquery: None,
        };

        let error = eval_query(jsonpath_query, &variables, json_http_response())
            .err()
            .unwrap();
        assert_eq!(
            error.source_info.start,
            Pos {
                line: 1,
                column: 10,
            }
        );
        assert_eq!(
            error.inner,
            RunnerError::QueryInvalidJsonpathExpression {
                value: "xxx".to_string()
            }
        );
    }

    #[test]
    fn test_query_invalid_json() {
        let variables = HashMap::new();
        let http_response = http::Response {
            version: http::Version::Http10,
            status: 0,
            headers: vec![],
            body: String::into_bytes(String::from("xxx")),
            duration: Default::default(),
        };
        let error = eval_query(jsonpath_success(), &variables, http_response)
            .err()
            .unwrap();
        assert_eq!(error.source_info.start, Pos { line: 1, column: 1 });
        assert_eq!(error.inner, RunnerError::QueryInvalidJson);
    }

    #[test]
    fn test_query_json_not_found() {
        let variables = HashMap::new();
        let http_response = http::Response {
            version: http::Version::Http10,
            status: 0,
            headers: vec![],
            body: String::into_bytes(String::from("{}")),
            duration: Default::default(),
        };
        //assert_eq!(jsonpath_success().eval(http_response).unwrap(), Value::List(vec![]));
        assert_eq!(
            eval_query(jsonpath_success(), &variables, http_response).unwrap(),
            None
        );
    }

    #[test]
    fn test_query_json() {
        let variables = HashMap::new();
        assert_eq!(
            eval_query(jsonpath_success(), &variables, json_http_response())
                .unwrap()
                .unwrap(),
            Value::Bool(false)
        );
        assert_eq!(
            eval_query(jsonpath_errors(), &variables, json_http_response())
                .unwrap()
                .unwrap(),
            Value::List(vec![
                Value::Object(vec![(
                    String::from("id"),
                    Value::String(String::from("error1"))
                )]),
                Value::Object(vec![(
                    String::from("id"),
                    Value::String(String::from("error2"))
                )])
            ])
        );

        assert_eq!(
            eval_query(jsonpath_errors_count(), &variables, json_http_response())
                .unwrap()
                .unwrap(),
            Value::Integer(2)
        );
    }

    #[test]
    fn test_query_regex() {
        let variables = HashMap::new();
        assert_eq!(
            eval_query(regex_name(), &variables, http::hello_http_response())
                .unwrap()
                .unwrap(),
            Value::String("World".to_string())
        );

        let error = eval_query(regex_invalid(), &variables, http::hello_http_response())
            .err()
            .unwrap();
        assert_eq!(error.source_info, SourceInfo::init(1, 7, 1, 10));
        assert_eq!(error.inner, RunnerError::InvalidRegex());
    }

    #[test]
    fn test_query_bytes() {
        let variables = HashMap::new();
        assert_eq!(
            eval_query(
                Query {
                    source_info: SourceInfo::init(0, 0, 0, 0),
                    value: QueryValue::Bytes {},
                    subquery: None
                },
                &variables,
                http::hello_http_response(),
            )
            .unwrap()
            .unwrap(),
            Value::Bytes(String::into_bytes(String::from("Hello World!")))
        );
    }

    #[test]
    fn test_query_sha256() {
        let variables = HashMap::new();
        assert_eq!(
            eval_query(
                Query {
                    source_info: SourceInfo::init(0, 0, 0, 0),
                    value: QueryValue::Sha256 {},
                    subquery: None
                },
                &variables,
                http::Response {
                    version: http::Version::Http10,
                    status: 200,
                    headers: vec![],
                    body: vec![0xff],
                    duration: Default::default(),
                },
            )
            .unwrap()
            .unwrap(),
            Value::Bytes(
                hex!("a8100ae6aa1940d0b663bb31cd466142ebbdbd5187131b92d93818987832eb89").to_vec()
            )
        );
    }
}
