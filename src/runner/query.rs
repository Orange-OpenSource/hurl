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

use std::collections::HashMap;

use regex::Regex;

use crate::core::common::Value;
use crate::http;
use crate::jsonpath;

use super::super::core::ast::*;
use super::cookie;
use super::core::{Error, RunnerError};
use super::xpath;

pub type QueryResult = Result<Option<Value>, Error>;

impl Query {
    pub fn eval(
        self,
        variables: &HashMap<String, Value>,
        http_response: http::Response,
    ) -> QueryResult {
        match self.value {
            QueryValue::Status {} => Ok(Some(Value::Integer(i64::from(http_response.status)))),
            QueryValue::Header { name, .. } => {
                let header_name = name.eval(variables)?;
                let values = http_response.get_header(header_name);
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
                let cookie_name = name.eval(variables)?;
                match http_response.get_cookie(cookie_name) {
                    None => Ok(None),
                    Some(cookie) => {
                        let attribute_name = if let Some(attribute) = attribute {
                            attribute.name
                        } else {
                            CookieAttributeName::Value("Value".to_string())
                        };
                        Ok(attribute_name.eval(cookie))
                    }
                }
            }
            QueryValue::Body {} => {
                // can return a string if encoding is known and utf8
                match http_response.text() {
                    Ok(s) => Ok(Some(Value::String(s))),
                    Err(inner) => Err(Error {
                        source_info: self.source_info.clone(),
                        inner,
                        assert: false,
                    }),
                }
            }
            QueryValue::Xpath { expr, .. } => {
                let source_info = expr.source_info.clone();
                let value = expr.eval(variables)?;
                match http_response.text() {
                    Err(inner) => Err(Error {
                        source_info: self.source_info.clone(),
                        inner,
                        assert: false,
                    }),
                    Ok(xml) => {
                        let result = if http_response.is_html() {
                            xpath::eval_html(xml, value.clone())
                        } else {
                            xpath::eval_xml(xml, value.clone())
                        };
                        match result {
                            Ok(value) => Ok(Some(value)),
                            Err(xpath::XpathError::InvalidXML {}) => Err(Error {
                                source_info: self.source_info,
                                inner: RunnerError::QueryInvalidXml,
                                assert: false,
                            }),
                            Err(xpath::XpathError::InvalidHtml {}) => Err(Error {
                                source_info: self.source_info,
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
                let value = expr.clone().eval(variables)?;
                let source_info = expr.source_info;
                //                let expr = match jsonpath::Expr::init(value.as_str()) {
                //                    None => return Err(Error { source_info: source_info.clone(), inner: RunnerError::QueryInvalidJsonpathExpression {}, assert: false }),
                //                    Some(expr) => expr
                //                };
                //                let json = match String::from_utf8(http_response.body) {
                //                    Err(_) => return Err(Error { source_info: self.source_info, inner: RunnerError::InvalidUtf8, assert: false }),
                //                    Ok(v) => v
                //                };
                //                let value = match expr.eval(json.as_str()) {
                //                    Err(_) => {
                //                        return Err(Error { source_info: self.source_info, inner: RunnerError::QueryInvalidJson, assert: false });
                //                    }
                //                    Ok(value) => {
                //                        if value == Value::List(vec![]) { Value::None } else { value }
                //                    }
                //                };
                // Using your own json implem
                let query = match jsonpath::parser::parse::parse(value.as_str()) {
                    Ok(q) => q,
                    Err(_) => {
                        return Err(Error {
                            source_info,
                            inner: RunnerError::QueryInvalidJsonpathExpression { value },
                            assert: false,
                        })
                    }
                };
                let json = match http_response.text() {
                    Err(inner) => {
                        return Err(Error {
                            source_info: self.source_info.clone(),
                            inner,
                            assert: false,
                        })
                    }
                    Ok(v) => v,
                };
                let value = match serde_json::from_str(json.as_str()) {
                    Err(_) => {
                        return Err(Error {
                            source_info: self.source_info,
                            inner: RunnerError::QueryInvalidJson,
                            assert: false,
                        });
                    }
                    Ok(v) => v,
                };
                let results = query.eval(value);
                if results.is_empty() {
                    Ok(None)
                } else if results.len() == 1 {
                    // list coercions
                    Ok(Some(Value::from_json(results.get(0).unwrap())))
                } else {
                    Ok(Some(Value::from_json(&serde_json::Value::Array(results))))
                }
            }
            QueryValue::Regex { expr, .. } => {
                let value = expr.clone().eval(variables)?;
                let source_info = expr.source_info;
                let s = match http_response.text() {
                    Err(inner) => {
                        return Err(Error {
                            source_info: self.source_info.clone(),
                            inner,
                            assert: false,
                        })
                    }
                    Ok(v) => v,
                };
                match Regex::new(value.as_str()) {
                    Ok(re) => match re.captures(s.as_str()) {
                        Some(captures) => match captures.get(1) {
                            Some(v) => Ok(Some(Value::String(v.as_str().to_string()))),
                            None => Ok(None),
                        },
                        None => Ok(None),
                    },
                    Err(_) => Err(Error {
                        source_info,
                        inner: RunnerError::InvalidRegex(),
                        assert: false,
                    }),
                }
            }
            QueryValue::Variable { name, .. } => {
                let name = name.eval(variables)?;
                if let Some(value) = variables.get(name.as_str()) {
                    Ok(Some(value.clone()))
                } else {
                    Ok(None)
                }
            }
        }
    }
}

impl CookieAttributeName {
    pub fn eval(self, cookie: cookie::ResponseCookie) -> Option<Value> {
        match self {
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
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::core::common::{Pos, SourceInfo};

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
                expr: Template {
                    quotes: true,
                    elements: vec![TemplateElement::String {
                        value: "Hello ([a-zA-Z]+)!".to_string(),
                        encoded: "Hello ([a-zA-Z]+)!".to_string(),
                    }],
                    source_info: SourceInfo::init(1, 7, 1, 26),
                },
            },
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
                expr: Template {
                    quotes: true,
                    elements: vec![TemplateElement::String {
                        value: "???".to_string(),
                        encoded: "???".to_string(),
                    }],
                    source_info: SourceInfo::init(1, 7, 1, 10),
                },
            },
        }
    }

    #[test]
    fn test_query_status() {
        let variables = HashMap::new();
        assert_eq!(
            Query {
                source_info: SourceInfo::init(0, 0, 0, 0),
                value: QueryValue::Status {}
            }
            .eval(&variables, http::hello_http_response())
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
        };
        //    let error = query_header.eval(http::hello_http_response()).err().unwrap();
        //    assert_eq!(error.source_info.start, Pos { line: 1, column: 8 });
        //    assert_eq!(error.inner, RunnerError::QueryHeaderNotFound);
        assert_eq!(
            query_header
                .eval(&variables, http::hello_http_response())
                .unwrap(),
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
        };
        assert_eq!(
            query_header
                .eval(&variables, http::hello_http_response())
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
        };
        assert_eq!(
            query.eval(&variables, response.clone()).unwrap().unwrap(),
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
        };
        assert_eq!(
            query.eval(&variables, response.clone()).unwrap().unwrap(),
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
        };
        assert_eq!(
            query.eval(&variables, response.clone()).unwrap().unwrap(),
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
        };
        assert_eq!(query.eval(&variables, response).unwrap(), None);
    }

    #[test]
    fn test_eval_cookie_attribute_name() {
        let cookie = cookie::ResponseCookie {
            name: "LSID".to_string(),
            value: "DQAAAKEaem_vYg".to_string(),
            attributes: vec![
                cookie::CookieAttribute {
                    name: "Path".to_string(),
                    value: Some("/accounts".to_string()),
                },
                cookie::CookieAttribute {
                    name: "Expires".to_string(),
                    value: Some("Wed, 13 Jan 2021 22:23:01 GMT".to_string()),
                },
                cookie::CookieAttribute {
                    name: "Secure".to_string(),
                    value: None,
                },
                cookie::CookieAttribute {
                    name: "HttpOnly".to_string(),
                    value: None,
                },
            ],
        };
        assert_eq!(
            CookieAttributeName::Value("_".to_string())
                .eval(cookie.clone())
                .unwrap(),
            Value::String("DQAAAKEaem_vYg".to_string())
        );
        assert_eq!(
            CookieAttributeName::Domain("_".to_string()).eval(cookie.clone()),
            None
        );
        assert_eq!(
            CookieAttributeName::Path("_".to_string())
                .eval(cookie.clone())
                .unwrap(),
            Value::String("/accounts".to_string())
        );
        assert_eq!(
            CookieAttributeName::MaxAge("_".to_string()).eval(cookie.clone()),
            None
        );
        assert_eq!(
            CookieAttributeName::Expires("_".to_string())
                .eval(cookie.clone())
                .unwrap(),
            Value::String("Wed, 13 Jan 2021 22:23:01 GMT".to_string())
        );
        assert_eq!(
            CookieAttributeName::Secure("_".to_string())
                .eval(cookie.clone())
                .unwrap(),
            Value::Unit
        );
        assert_eq!(
            CookieAttributeName::HttpOnly("_".to_string())
                .eval(cookie.clone())
                .unwrap(),
            Value::Unit
        );
        assert_eq!(
            CookieAttributeName::SameSite("_".to_string()).eval(cookie),
            None
        );
    }

    #[test]
    fn test_body() {
        let variables = HashMap::new();
        assert_eq!(
            Query {
                source_info: SourceInfo::init(0, 0, 0, 0),
                value: QueryValue::Body {},
            }
            .eval(&variables, http::hello_http_response())
            .unwrap()
            .unwrap(),
            Value::String(String::from("Hello World!"))
        );
        let error = Query {
            source_info: SourceInfo::init(1, 1, 1, 2),
            value: QueryValue::Body {},
        }
        .eval(&variables, http::bytes_http_response())
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
        };
        let error = xpath_users().eval(&variables, http_response).err().unwrap();
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
        };
        let error = query
            .eval(&variables, http::xml_two_users_http_response())
            .err()
            .unwrap();
        assert_eq!(error.inner, RunnerError::QueryInvalidXpathEval);
        assert_eq!(error.source_info.start, Pos { line: 1, column: 7 });
    }

    #[test]
    fn test_query_xpath() {
        let variables = HashMap::new();

        assert_eq!(
            xpath_users()
                .eval(&variables, http::xml_two_users_http_response())
                .unwrap()
                .unwrap(),
            Value::Nodeset(2)
        );
        assert_eq!(
            xpath_count_user_query()
                .eval(&variables, http::xml_two_users_http_response())
                .unwrap()
                .unwrap(),
            Value::Float(2, 0)
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
        }
    }

    #[test]
    fn test_query_xpath_with_html() {
        let variables = HashMap::new();
        assert_eq!(
            xpath_html_charset()
                .eval(&variables, http::html_http_response())
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
        };

        let error = jsonpath_query
            .eval(&variables, json_http_response())
            .err()
            .unwrap();
        assert_eq!(
            error.source_info.start,
            Pos {
                line: 1,
                column: 10
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
        };
        let error = jsonpath_success()
            .eval(&variables, http_response)
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
        };
        //assert_eq!(jsonpath_success().eval(http_response).unwrap(), Value::List(vec![]));
        assert_eq!(
            jsonpath_success().eval(&variables, http_response).unwrap(),
            None
        );
    }

    #[test]
    fn test_query_json() {
        let variables = HashMap::new();
        assert_eq!(
            jsonpath_success()
                .eval(&variables, json_http_response())
                .unwrap()
                .unwrap(),
            Value::Bool(false)
        );
        assert_eq!(
            jsonpath_errors()
                .eval(&variables, json_http_response())
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
    }

    #[test]
    fn test_query_regex() {
        let variables = HashMap::new();
        assert_eq!(
            regex_name()
                .eval(&variables, http::hello_http_response())
                .unwrap()
                .unwrap(),
            Value::String("World".to_string())
        );

        let error = regex_invalid()
            .eval(&variables, http::hello_http_response())
            .err()
            .unwrap();
        assert_eq!(error.source_info, SourceInfo::init(1, 7, 1, 10));
        assert_eq!(error.inner, RunnerError::InvalidRegex());
    }
}
