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
use chrono::Utc;
use hurl_core::ast::{
    CertificateAttributeName, CookieAttribute, CookieAttributeName, CookiePath, Query, QueryValue,
    RegexValue, SourceInfo, Template,
};
use regex::Regex;
use sha2::Digest;

use crate::http::{HttpError, Response, ResponseCookie};

use super::cache::BodyCache;
use super::error::{RunnerError, RunnerErrorKind};
use super::filter;
use super::http_response::HttpResponse;
use super::number::Number;
use super::template::eval_template;
use super::value::Value;
use super::variable::VariableSet;
use super::xpath::{Document, Format};

pub type QueryResult = Result<Option<Value>, RunnerError>;

/// Evaluates this `query` and returns a [`QueryResult`], using a list of HTTP `responses` and `variables`.
pub fn eval_query(
    query: &Query,
    variables: &VariableSet,
    responses: &[&Response],
    cache: &mut BodyCache,
) -> QueryResult {
    let last_response = responses.last().unwrap();
    match &query.value {
        QueryValue::Status => eval_query_status(last_response),
        QueryValue::Version => eval_query_version(last_response),
        QueryValue::Url => eval_query_url(last_response),
        QueryValue::Header { name, .. } => eval_query_header(last_response, name, variables),
        QueryValue::Cookie {
            expr: CookiePath { name, attribute },
            ..
        } => eval_query_cookie(last_response, name, attribute, variables),
        QueryValue::Body => eval_query_body(last_response, query.source_info),
        QueryValue::Xpath { expr, .. } => {
            eval_query_xpath(last_response, cache, expr, variables, query.source_info)
        }
        QueryValue::Jsonpath { expr, .. } => {
            eval_query_jsonpath(last_response, cache, expr, variables, query.source_info)
        }
        QueryValue::Regex { value, .. } => {
            eval_query_regex(last_response, value, variables, query.source_info)
        }
        QueryValue::Variable { name, .. } => eval_query_variable(name, variables),
        QueryValue::Duration => eval_query_duration(last_response),
        QueryValue::Bytes => eval_query_bytes(last_response, query.source_info),
        QueryValue::Sha256 => eval_query_sha256(last_response, query.source_info),
        QueryValue::Md5 => eval_query_md5(last_response, query.source_info),
        QueryValue::Certificate {
            attribute_name: field,
            ..
        } => eval_query_certificate(last_response, *field),
        QueryValue::Ip => eval_ip(last_response),
        QueryValue::Redirects => eval_redirects(responses),
    }
}

/// Evaluates the response status code using the HTTP `response`.
fn eval_query_status(response: &Response) -> QueryResult {
    Ok(Some(Value::Number(Number::Integer(i64::from(
        response.status,
    )))))
}

/// Evaluates the version on the HTTP `response`
fn eval_query_version(response: &Response) -> QueryResult {
    Ok(Some(Value::String(
        response
            .version
            .to_string()
            .strip_prefix("HTTP/")
            .unwrap()
            .to_string(),
    )))
}

/// Evaluates the final URL of the HTTP `response`.
fn eval_query_url(response: &Response) -> QueryResult {
    Ok(Some(Value::String(response.url.to_string())))
}

/// Evaluates a response query header `name`, on the HTTP `response` given a set of `variables`.
fn eval_query_header(response: &Response, name: &Template, variables: &VariableSet) -> QueryResult {
    let name = eval_template(name, variables)?;
    let values = response.headers.values(&name);
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

/// Evaluates a cookie query `name` with optional attributes, on the HTTP `response` given a set of `variables`.
fn eval_query_cookie(
    response: &Response,
    name: &Template,
    attribute: &Option<CookieAttribute>,
    variables: &VariableSet,
) -> QueryResult {
    let query_source_info = name.source_info;
    let name = eval_template(name, variables)?;
    match response.get_cookie(&name) {
        None => Ok(None),
        Some(cookie) => {
            let attribute_name = if let Some(attribute) = attribute {
                attribute.name.clone()
            } else {
                CookieAttributeName::Value("Value".to_string())
            };
            eval_cookie_attribute_name(attribute_name, cookie, query_source_info)
        }
    }
}

/// Evaluates the HTTP `response` body as text.
///
/// `query_source_info` is the source position of the query, used if an error is returned.
fn eval_query_body(response: &Response, query_source_info: SourceInfo) -> QueryResult {
    // Can return a string if encoding is known and utf8.
    match response.text() {
        Ok(s) => Ok(Some(Value::String(s))),
        Err(inner) => Err(RunnerError::new(
            query_source_info,
            RunnerErrorKind::Http(inner),
            false,
        )),
    }
}

/// Evaluates a XPath expression on the HTTP `response` body, given a set of `variables`.
///
/// `query_source_info` is the source position of the query, used if an error is returned.
fn eval_query_xpath(
    response: &Response,
    cache: &mut BodyCache,
    expr: &Template,
    variables: &VariableSet,
    query_source_info: SourceInfo,
) -> QueryResult {
    let doc = match cache.xml() {
        Some(d) => d,
        None => parse_cache_xml(response, cache, query_source_info)?,
    };
    filter::eval_xpath_doc(doc, expr, variables)
}

/// Parse this HTTP `response` body to a structured XML document, and store the document to the
/// response `cache`.
///
/// `query_source_info` is used for error reporting.
fn parse_cache_xml<'cache>(
    response: &Response,
    cache: &'cache mut BodyCache,
    query_source_info: SourceInfo,
) -> Result<&'cache Document, RunnerError> {
    // Get the response as text if possible
    let text = match response.text() {
        Ok(t) => t,
        Err(e) => {
            return Err(RunnerError::new(
                query_source_info,
                RunnerErrorKind::Http(e),
                false,
            ))
        }
    };
    let format = if response.is_html() {
        Format::Html
    } else {
        Format::Xml
    };
    let Ok(doc) = Document::parse(&text, format) else {
        return Err(RunnerError::new(
            query_source_info,
            RunnerErrorKind::QueryInvalidXml,
            false,
        ));
    };
    // Everything is ok, we can put the response in the cache
    cache.set_xml(doc);
    Ok(cache.xml().unwrap())
}

/// Evaluates a JSONPath expression on the HTTP `response` body, given a set of `variables`.
///
/// `query_source_info` is the source position of the query, used if an error is returned.
fn eval_query_jsonpath(
    response: &Response,
    cache: &mut BodyCache,
    expr: &Template,
    variables: &VariableSet,
    query_source_info: SourceInfo,
) -> QueryResult {
    let json = match cache.json() {
        Some(j) => j,
        None => parse_cache_json(response, cache, query_source_info)?,
    };
    filter::eval_jsonpath_json(json, expr, variables)
}

/// Parse this HTTP `response` body to JSON, and store the document to the response `cache`.
///
/// `query_source_info` is used for error reporting.
fn parse_cache_json<'cache>(
    response: &Response,
    cache: &'cache mut BodyCache,
    query_source_info: SourceInfo,
) -> Result<&'cache serde_json::Value, RunnerError> {
    // Get the response as text if possible
    let text = match response.text() {
        Ok(t) => t,
        Err(e) => {
            return Err(RunnerError::new(
                query_source_info,
                RunnerErrorKind::Http(e),
                false,
            ))
        }
    };
    let json = match serde_json::from_str(&text) {
        Err(_) => {
            return Err(RunnerError::new(
                query_source_info,
                RunnerErrorKind::QueryInvalidJson,
                false,
            ));
        }
        Ok(v) => v,
    };
    // Everything is ok, we can put the response in the cache
    cache.set_json(json);
    Ok(cache.json().unwrap())
}

/// Evaluates a regex query on the HTTP `response` body, given a set of `variables`.
///
/// `query_source_info` is the source position of the query, used if an error is returned.
fn eval_query_regex(
    response: &Response,
    regex: &RegexValue,
    variables: &VariableSet,
    query_source_info: SourceInfo,
) -> QueryResult {
    let s = match response.text() {
        Ok(v) => v,
        Err(inner) => {
            return Err(RunnerError::new(
                query_source_info,
                RunnerErrorKind::Http(inner),
                false,
            ))
        }
    };
    let re = match regex {
        RegexValue::Template(t) => {
            let value = eval_template(t, variables)?;
            match Regex::new(value.as_str()) {
                Ok(re) => re,
                Err(_) => {
                    return Err(RunnerError::new(
                        t.source_info,
                        RunnerErrorKind::InvalidRegex,
                        false,
                    ))
                }
            }
        }
        RegexValue::Regex(re) => re.inner.clone(),
    };
    match re.captures(s.as_str()) {
        Some(captures) => match captures.get(1) {
            Some(v) => Ok(Some(Value::String(v.as_str().to_string()))),
            None => Ok(None),
        },
        None => Ok(None),
    }
}

/// Evaluates a variable, given a set of `variables`.
fn eval_query_variable(name: &Template, variables: &VariableSet) -> QueryResult {
    let name = eval_template(name, variables)?;
    if let Some(variable) = variables.get(&name) {
        Ok(Some(variable.value().clone()))
    } else {
        Ok(None)
    }
}

/// Evaluates the effective duration of the HTTP `response` (only transfer time, assert and captures
/// are not taken into account).
fn eval_query_duration(response: &Response) -> QueryResult {
    Ok(Some(Value::Number(Number::Integer(
        response.duration.as_millis() as i64,
    ))))
}

/// Evaluates the HTTP `response` body as bytes.
///
/// `query_source_info` is the source position of the query, used if an error is returned.
fn eval_query_bytes(response: &Response, query_source_info: SourceInfo) -> QueryResult {
    match response.uncompress_body() {
        Ok(s) => Ok(Some(Value::Bytes(s))),
        Err(inner) => Err(RunnerError::new(
            query_source_info,
            RunnerErrorKind::Http(inner),
            false,
        )),
    }
}

/// Evaluates the SHA-256 hash of the HTTP `response` body bytes.
///
/// `query_source_info` is the source position of the query, used if an error is returned.
fn eval_query_sha256(response: &Response, query_source_info: SourceInfo) -> QueryResult {
    let bytes = match response.uncompress_body() {
        Ok(s) => s,
        Err(inner) => {
            return Err(RunnerError::new(
                query_source_info,
                RunnerErrorKind::Http(inner),
                false,
            ));
        }
    };
    let mut hasher = sha2::Sha256::new();
    hasher.update(bytes);
    let result = hasher.finalize();
    let bytes = Value::Bytes(result[..].to_vec());
    Ok(Some(bytes))
}

/// Evaluates the MD-5 hash of the HTTP `response` body bytes.
///
/// `query_source_info` is the source position of the query, used if an error is returned.
fn eval_query_md5(response: &Response, query_source_info: SourceInfo) -> QueryResult {
    let bytes = match response.uncompress_body() {
        Ok(s) => s,
        Err(inner) => {
            return Err(RunnerError::new(
                query_source_info,
                RunnerErrorKind::Http(inner),
                false,
            ));
        }
    };
    let bytes = md5::compute(bytes).to_vec();
    Ok(Some(Value::Bytes(bytes)))
}

/// Evaluates the SSL certificate attribute, of the HTTP `response`.
fn eval_query_certificate(
    response: &Response,
    certificate_attribute: CertificateAttributeName,
) -> QueryResult {
    if let Some(certificate) = &response.certificate {
        let value = match certificate_attribute {
            CertificateAttributeName::Subject => Value::String(certificate.subject.clone()),
            CertificateAttributeName::Issuer => Value::String(certificate.issuer.clone()),
            CertificateAttributeName::StartDate => Value::Date(certificate.start_date),
            CertificateAttributeName::ExpireDate => Value::Date(certificate.expire_date),
            CertificateAttributeName::SerialNumber => {
                Value::String(certificate.serial_number.clone())
            }
        };
        Ok(Some(value))
    } else {
        Ok(None)
    }
}

/// Evaluates the ip address of the HTTP `response`.
fn eval_ip(response: &Response) -> QueryResult {
    Ok(Some(Value::String(response.ip_addr.to_string())))
}

/// Evaluates the redirects within a list of HTTP `responses`
fn eval_redirects(responses: &[&Response]) -> QueryResult {
    let mut it = responses.iter().peekable();
    let mut values: Vec<Value> = vec![];
    while let Some(r) = it.next() {
        // We peek the next response in the response chain to use the next URL response
        // as the current response location. The URLs' response chain has already been resolved
        // we don't need to reconstruct the location from response headers.
        let location = it.peek().map(|r| r.url.clone());
        // We're only interested to redirection:
        if location.is_some() {
            let response = Value::HttpResponse(HttpResponse::new(location, r.status));
            values.push(response);
        }
    }
    Ok(Some(Value::List(values)))
}

fn eval_cookie_attribute_name(
    cookie_attribute_name: CookieAttributeName,
    cookie: ResponseCookie,
    query_source_info: SourceInfo,
) -> QueryResult {
    match cookie_attribute_name {
        CookieAttributeName::Value(_) => Ok(Some(Value::String(cookie.value))),
        CookieAttributeName::Expires(_) => {
            if let Some(s) = cookie.expires() {
                if let Ok(v) = chrono::DateTime::parse_from_rfc2822(s.as_str()) {
                    Ok(Some(Value::Date(v.with_timezone(&chrono::Utc))))

                // support format with dash such as Wed, 13-Jan-2021 22:23:01 GMT
                // TODO: search for for other possible date format used in the wild
                } else if let Ok(v) = chrono::NaiveDateTime::parse_from_str(
                    s.as_str(),
                    "%a, %d-%b-%Y %H:%M:%S%.3f GMT",
                ) {
                    Ok(Some(Value::Date(v.and_local_timezone(Utc).unwrap())))
                } else {
                    Err(RunnerError::new(
                        query_source_info,
                        RunnerErrorKind::Http(HttpError::CouldNotParseCookieExpires(s)),
                        true,
                    ))
                }
            } else {
                Ok(None)
            }
        }
        CookieAttributeName::MaxAge(_) => {
            Ok(cookie.max_age().map(|v| Value::Number(Number::Integer(v))))
        }
        CookieAttributeName::Domain(_) => Ok(cookie.domain().map(Value::String)),
        CookieAttributeName::Path(_) => Ok(cookie.path().map(Value::String)),
        CookieAttributeName::Secure(_) => {
            if cookie.has_secure() {
                Ok(Some(Value::Unit))
            } else {
                Ok(None)
            }
        }
        CookieAttributeName::HttpOnly(_) => {
            if cookie.has_httponly() {
                Ok(Some(Value::Unit))
            } else {
                Ok(None)
            }
        }
        CookieAttributeName::SameSite(_) => Ok(cookie.samesite().map(Value::String)),
    }
}

impl Value {
    pub fn from_json(value: &serde_json::Value) -> Value {
        match value {
            serde_json::Value::Null => Value::Null,
            serde_json::Value::Bool(bool) => Value::Bool(*bool),
            serde_json::Value::Number(n) => {
                if n.is_f64() {
                    Value::Number(Number::from(n.as_f64().unwrap()))
                } else if n.is_i64() {
                    Value::Number(Number::from(n.as_i64().unwrap()))
                } else {
                    Value::Number(Number::BigInteger(n.to_string()))
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
    use std::num::ParseIntError;

    use hurl_core::ast::{SourceInfo, TemplateElement, Whitespace};
    use hurl_core::reader::Pos;
    use hurl_core::types::ToSource;

    use super::*;
    use crate::http;
    use crate::http::{HeaderVec, HttpError, HttpVersion};

    fn default_response() -> Response {
        Response {
            version: HttpVersion::Http10,
            status: 200,
            headers: HeaderVec::new(),
            body: vec![],
            duration: Default::default(),
            url: "http://localhost".parse().unwrap(),
            certificate: None,
            ip_addr: Default::default(),
        }
    }

    pub fn xpath_invalid_query() -> Query {
        // xpath ???
        let whitespace = Whitespace {
            value: String::from(" "),
            source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
        };
        Query {
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 13)),
            value: QueryValue::Xpath {
                space0: whitespace,
                expr: Template::new(
                    Some('"'),
                    vec![TemplateElement::String {
                        value: "???".to_string(),
                        source: "???".to_source(),
                    }],
                    SourceInfo::new(Pos::new(1, 7), Pos::new(1, 10)),
                ),
            },
        }
    }

    pub fn xpath_count_user_query() -> Query {
        let whitespace = Whitespace {
            value: String::from(" "),
            source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
        };
        Query {
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 13)),
            value: QueryValue::Xpath {
                space0: whitespace,
                expr: Template::new(
                    Some('"'),
                    vec![TemplateElement::String {
                        value: "count(//user)".to_string(),
                        source: "count(//user)".to_source(),
                    }],
                    SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                ),
            },
        }
    }

    pub fn xpath_users() -> Query {
        let whitespace = Whitespace {
            value: String::from(" "),
            source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
        };
        Query {
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 13)),
            value: QueryValue::Xpath {
                space0: whitespace,
                expr: Template::new(
                    Some('"'),
                    vec![TemplateElement::String {
                        value: "//user".to_string(),
                        source: "/user".to_source(),
                    }],
                    SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                ),
            },
        }
    }

    pub fn jsonpath_success() -> Query {
        // jsonpath $.success
        Query {
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 19)),
            value: QueryValue::Jsonpath {
                space0: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 9), Pos::new(1, 10)),
                },
                expr: Template::new(
                    Some('"'),
                    vec![TemplateElement::String {
                        value: "$.success".to_string(),
                        source: "$.success".to_source(),
                    }],
                    SourceInfo::new(Pos::new(1, 10), Pos::new(1, 19)),
                ),
            },
        }
    }

    pub fn jsonpath_errors() -> Query {
        // jsonpath $.errors
        Query {
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 19)),
            value: QueryValue::Jsonpath {
                space0: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 9), Pos::new(1, 10)),
                },
                expr: Template::new(
                    Some('"'),
                    vec![TemplateElement::String {
                        value: "$.errors".to_string(),
                        source: "$.errors".to_source(),
                    }],
                    SourceInfo::new(Pos::new(1, 10), Pos::new(1, 18)),
                ),
            },
        }
    }

    pub fn jsonpath_duration() -> Query {
        // jsonpath $.errors
        Query {
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 19)),
            value: QueryValue::Jsonpath {
                space0: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 9), Pos::new(1, 10)),
                },
                expr: Template::new(
                    Some('"'),
                    vec![TemplateElement::String {
                        value: "$.duration".to_string(),
                        source: "$.duration".to_source(),
                    }],
                    SourceInfo::new(Pos::new(1, 10), Pos::new(1, 18)),
                ),
            },
        }
    }

    pub fn regex_name() -> Query {
        // regex "Hello ([a-zA-Z]+)!"
        Query {
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 26)),
            value: QueryValue::Regex {
                space0: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 6), Pos::new(1, 7)),
                },
                value: RegexValue::Template(Template::new(
                    Some('"'),
                    vec![TemplateElement::String {
                        value: "Hello ([a-zA-Z]+)!".to_string(),
                        source: "Hello ([a-zA-Z]+)!".to_source(),
                    }],
                    SourceInfo::new(Pos::new(1, 7), Pos::new(1, 26)),
                )),
            },
        }
    }

    pub fn regex_invalid() -> Query {
        // regex ???"
        Query {
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 26)),
            value: QueryValue::Regex {
                space0: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 6), Pos::new(1, 7)),
                },
                value: RegexValue::Template(Template::new(
                    Some('"'),
                    vec![TemplateElement::String {
                        value: "???".to_string(),
                        source: "???".to_source(),
                    }],
                    SourceInfo::new(Pos::new(1, 7), Pos::new(1, 10)),
                )),
            },
        }
    }

    #[test]
    pub fn value_from_json() {
        let json_number: serde_json::Value = serde_json::from_str("1000").unwrap();
        assert_eq!(
            Value::from_json(&json_number),
            Value::Number(Number::Integer(1000))
        );

        let json_number: serde_json::Value = serde_json::from_str("1.0").unwrap();
        assert_eq!(
            Value::from_json(&json_number),
            Value::Number(Number::Float(1.0))
        );

        let json_number: serde_json::Value =
            serde_json::from_str("1000000000000000000000").unwrap();
        assert_eq!(
            Value::from_json(&json_number),
            Value::Number(Number::BigInteger("1000000000000000000000".to_string()))
        );

        let json_number: serde_json::Value =
            serde_json::from_str("1000000000000000000000.5").unwrap();
        assert_eq!(
            Value::from_json(&json_number),
            Value::Number(Number::Float(1000000000000000000000.5f64))
        );
    }

    #[test]
    fn test_query_status() {
        let variables = VariableSet::new();
        let mut cache = BodyCache::new();
        assert_eq!(
            eval_query(
                &Query {
                    source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                    value: QueryValue::Status,
                },
                &variables,
                &[&http::hello_http_response()],
                &mut cache,
            )
            .unwrap()
            .unwrap(),
            Value::Number(Number::Integer(200))
        );
    }

    #[test]
    fn test_header_not_found() {
        let variables = VariableSet::new();
        let mut cache = BodyCache::new();

        // header Custom
        let query_header = Query {
            source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
            value: QueryValue::Header {
                space0: Whitespace {
                    value: String::from(" "),
                    source_info: SourceInfo::new(Pos::new(1, 7), Pos::new(1, 8)),
                },
                name: Template::new(
                    Some('"'),
                    vec![TemplateElement::String {
                        value: "Custom".to_string(),
                        source: "Custom".to_source(),
                    }],
                    SourceInfo::new(Pos::new(2, 8), Pos::new(2, 14)),
                ),
            },
        };
        //    let error = query_header.eval(http::hello_http_response()).err().unwrap();
        //    assert_eq!(error.source_info.start, Pos { line: 1, column: 8 });
        //    assert_eq!(error.inner, RunnerError::QueryHeaderNotFound);
        assert_eq!(
            eval_query(
                &query_header,
                &variables,
                &[&http::hello_http_response()],
                &mut cache
            )
            .unwrap(),
            None
        );
    }

    #[test]
    fn test_header() {
        // header Content-Type
        let variables = VariableSet::new();
        let mut cache = BodyCache::new();

        let query_header = Query {
            source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
            value: QueryValue::Header {
                space0: Whitespace {
                    value: String::from(" "),
                    source_info: SourceInfo::new(Pos::new(1, 7), Pos::new(1, 8)),
                },
                name: Template::new(
                    Some('"'),
                    vec![TemplateElement::String {
                        value: "Content-Type".to_string(),
                        source: "Content-Type".to_source(),
                    }],
                    SourceInfo::new(Pos::new(1, 8), Pos::new(1, 16)),
                ),
            },
        };
        assert_eq!(
            eval_query(
                &query_header,
                &variables,
                &[&http::hello_http_response()],
                &mut cache
            )
            .unwrap()
            .unwrap(),
            Value::String(String::from("text/html; charset=utf-8"))
        );
    }

    #[test]
    fn test_query_cookie() {
        let variables = VariableSet::new();
        let mut cache = BodyCache::new();

        let space = Whitespace {
            value: String::new(),
            source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
        };
        let mut headers = HeaderVec::new();
        headers.push(http::Header::new("Set-Cookie", "LSID=DQAAAKEaem_vYg; Path=/accounts; Expires=Wed, 13 Jan 2021 22:23:01 GMT; Secure; HttpOnly"));

        let response = Response {
            headers,
            ..default_response()
        };

        let responses = vec![&response];

        // cookie "LSID"
        let query = Query {
            source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
            value: QueryValue::Cookie {
                space0: space.clone(),
                expr: CookiePath {
                    name: Template::new(
                        Some('"'),
                        vec![TemplateElement::String {
                            value: "LSID".to_string(),
                            source: "LSID".to_source(),
                        }],
                        SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                    ),
                    attribute: None,
                },
            },
        };
        assert_eq!(
            eval_query(&query, &variables, &responses, &mut cache)
                .unwrap()
                .unwrap(),
            Value::String("DQAAAKEaem_vYg".to_string())
        );

        // cookie "LSID[Path]"
        let query = Query {
            source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
            value: QueryValue::Cookie {
                space0: space.clone(),
                expr: CookiePath {
                    name: Template::new(
                        Some('"'),
                        vec![TemplateElement::String {
                            value: "LSID".to_string(),
                            source: "LSID".to_source(),
                        }],
                        SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                    ),
                    attribute: Some(CookieAttribute {
                        space0: space.clone(),
                        name: CookieAttributeName::Path("Path".to_string()),
                        space1: space.clone(),
                    }),
                },
            },
        };
        assert_eq!(
            eval_query(&query, &variables, &responses, &mut cache)
                .unwrap()
                .unwrap(),
            Value::String("/accounts".to_string())
        );

        // cookie "LSID[Secure]"
        let query = Query {
            source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
            value: QueryValue::Cookie {
                space0: space.clone(),
                expr: CookiePath {
                    name: Template::new(
                        Some('"'),
                        vec![TemplateElement::String {
                            value: "LSID".to_string(),
                            source: "LSID".to_source(),
                        }],
                        SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                    ),
                    attribute: Some(CookieAttribute {
                        space0: space.clone(),
                        name: CookieAttributeName::Secure("Secure".to_string()),
                        space1: space.clone(),
                    }),
                },
            },
        };
        assert_eq!(
            eval_query(&query, &variables, &responses, &mut cache)
                .unwrap()
                .unwrap(),
            Value::Unit
        );

        // cookie "LSID[Domain]"
        let query = Query {
            source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
            value: QueryValue::Cookie {
                space0: space.clone(),
                expr: CookiePath {
                    name: Template::new(
                        Some('"'),
                        vec![TemplateElement::String {
                            value: "LSID".to_string(),
                            source: "LSID".to_source(),
                        }],
                        SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                    ),
                    attribute: Some(CookieAttribute {
                        space0: space.clone(),
                        name: CookieAttributeName::Domain("Domain".to_string()),
                        space1: space,
                    }),
                },
            },
        };
        assert_eq!(
            eval_query(&query, &variables, &responses, &mut cache).unwrap(),
            None
        );
    }

    #[test]
    fn test_eval_cookie_attribute_name() {
        let cookie = ResponseCookie {
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
        let query_source_info = SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0));

        assert_eq!(
            eval_cookie_attribute_name(
                CookieAttributeName::Value("_".to_string()),
                cookie.clone(),
                query_source_info
            )
            .unwrap()
            .unwrap(),
            Value::String("DQAAAKEaem_vYg".to_string())
        );
        assert_eq!(
            eval_cookie_attribute_name(
                CookieAttributeName::Domain("_".to_string()),
                cookie.clone(),
                query_source_info
            )
            .unwrap(),
            None
        );
        assert_eq!(
            eval_cookie_attribute_name(
                CookieAttributeName::Path("_".to_string()),
                cookie.clone(),
                query_source_info
            )
            .unwrap()
            .unwrap(),
            Value::String("/accounts".to_string())
        );
        assert_eq!(
            eval_cookie_attribute_name(
                CookieAttributeName::MaxAge("_".to_string()),
                cookie.clone(),
                query_source_info
            )
            .unwrap(),
            None
        );
        assert_eq!(
            eval_cookie_attribute_name(
                CookieAttributeName::Expires("_".to_string()),
                cookie.clone(),
                query_source_info
            )
            .unwrap()
            .unwrap(),
            Value::Date(
                chrono::DateTime::parse_from_rfc2822("Wed, 13 Jan 2021 22:23:01 GMT")
                    .unwrap()
                    .with_timezone(&chrono::Utc)
            ),
        );
        assert_eq!(
            eval_cookie_attribute_name(
                CookieAttributeName::Secure("_".to_string()),
                cookie.clone(),
                query_source_info
            )
            .unwrap()
            .unwrap(),
            Value::Unit
        );
        assert_eq!(
            eval_cookie_attribute_name(
                CookieAttributeName::HttpOnly("_".to_string()),
                cookie.clone(),
                query_source_info
            )
            .unwrap()
            .unwrap(),
            Value::Unit
        );
        assert_eq!(
            eval_cookie_attribute_name(
                CookieAttributeName::SameSite("_".to_string()),
                cookie,
                query_source_info
            )
            .unwrap(),
            None,
        );
    }

    #[test]
    fn test_eval_cookie_attribute_expires_error() {
        let cookie = ResponseCookie {
            name: "cookie1".to_string(),
            value: "value1".to_string(),
            attributes: vec![http::CookieAttribute {
                name: "Expires".to_string(),
                value: Some("???".to_string()),
            }],
        };
        let query_source_info = SourceInfo::new(Pos::new(5, 1), Pos::new(5, 20));
        let error = eval_cookie_attribute_name(
            CookieAttributeName::Expires("_".to_string()),
            cookie.clone(),
            query_source_info,
        )
        .unwrap_err();
        assert_eq!(
            error.source_info,
            SourceInfo::new(Pos::new(5, 1), Pos::new(5, 20))
        );
        assert_eq!(
            error.kind,
            RunnerErrorKind::Http(HttpError::CouldNotParseCookieExpires("???".to_string()))
        );
    }

    #[test]
    fn test_body() {
        let variables = VariableSet::new();
        let mut cache = BodyCache::new();

        assert_eq!(
            eval_query(
                &Query {
                    source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                    value: QueryValue::Body,
                },
                &variables,
                &[&http::hello_http_response()],
                &mut cache,
            )
            .unwrap()
            .unwrap(),
            Value::String(String::from("Hello World!"))
        );
        let error = eval_query(
            &Query {
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 2)),
                value: QueryValue::Body,
            },
            &variables,
            &[&http::bytes_http_response()],
            &mut cache,
        )
        .err()
        .unwrap();
        assert_eq!(
            error.source_info,
            SourceInfo::new(Pos::new(1, 1), Pos::new(1, 2))
        );
        assert_eq!(
            error.kind,
            RunnerErrorKind::Http(HttpError::InvalidDecoding {
                charset: "UTF-8".to_string()
            })
        );
    }

    #[test]
    fn test_query_invalid_utf8() {
        let variables = VariableSet::new();
        let mut cache = BodyCache::new();

        let http_response = Response {
            body: vec![200],
            ..default_response()
        };
        let error = eval_query(&xpath_users(), &variables, &[&http_response], &mut cache)
            .err()
            .unwrap();
        assert_eq!(error.source_info.start, Pos { line: 1, column: 1 });
        assert_eq!(
            error.kind,
            RunnerErrorKind::Http(HttpError::InvalidDecoding {
                charset: "UTF-8".to_string()
            })
        );
    }

    #[test]
    fn test_query_xpath_error_eval() {
        let variables = VariableSet::new();
        let mut cache = BodyCache::new();

        // xpath ^^^
        let query = Query {
            source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
            value: QueryValue::Xpath {
                space0: Whitespace {
                    value: String::from(" "),
                    source_info: SourceInfo::new(Pos::new(1, 6), Pos::new(1, 7)),
                },
                expr: Template::new(
                    Some('"'),
                    vec![TemplateElement::String {
                        value: "^^^".to_string(),
                        source: "^^^".to_source(),
                    }],
                    SourceInfo::new(Pos::new(1, 7), Pos::new(1, 10)),
                ),
            },
        };
        let error = eval_query(
            &query,
            &variables,
            &[&http::xml_two_users_http_response()],
            &mut cache,
        )
        .unwrap_err();
        assert_eq!(error.kind, RunnerErrorKind::InvalidXPathEval);
        assert_eq!(error.source_info.start, Pos { line: 1, column: 7 });
    }

    #[test]
    fn test_query_xpath() {
        let variables = VariableSet::new();
        let mut cache = BodyCache::new();

        assert_eq!(
            eval_query(
                &xpath_users(),
                &variables,
                &[&http::xml_two_users_http_response()],
                &mut cache,
            )
            .unwrap()
            .unwrap(),
            Value::Nodeset(2)
        );
        assert_eq!(
            eval_query(
                &xpath_count_user_query(),
                &variables,
                &[&http::xml_two_users_http_response()],
                &mut cache,
            )
            .unwrap()
            .unwrap(),
            Value::Number(Number::Float(2.0))
        );
    }

    #[cfg(test)]
    pub fn xpath_html_charset() -> Query {
        // $x("normalize-space(/html/head/meta/@charset)")
        let whitespace = Whitespace {
            value: String::from(" "),
            source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
        };
        Query {
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 13)),
            value: QueryValue::Xpath {
                space0: whitespace,
                expr: Template::new(
                    Some('"'),
                    vec![TemplateElement::String {
                        value: "normalize-space(/html/head/meta/@charset)".to_string(),
                        source: "normalize-space(/html/head/meta/@charset)".to_source(),
                    }],
                    SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                ),
            },
        }
    }

    #[test]
    fn test_query_xpath_with_html() {
        let variables = VariableSet::new();
        let mut cache = BodyCache::new();

        assert_eq!(
            eval_query(
                &xpath_html_charset(),
                &variables,
                &[&http::html_http_response()],
                &mut cache,
            )
            .unwrap()
            .unwrap(),
            Value::String(String::from("UTF-8"))
        );
    }

    #[test]
    fn test_query_jsonpath_invalid_expression() {
        let variables = VariableSet::new();
        let mut cache = BodyCache::new();

        // jsonpath xxx
        let jsonpath_query = Query {
            source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
            value: QueryValue::Jsonpath {
                space0: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 9), Pos::new(1, 10)),
                },
                expr: Template::new(
                    Some('"'),
                    vec![TemplateElement::String {
                        value: "xxx".to_string(),
                        source: "xxx".to_source(),
                    }],
                    SourceInfo::new(Pos::new(1, 10), Pos::new(1, 13)),
                ),
            },
        };

        let error = eval_query(
            &jsonpath_query,
            &variables,
            &[&http::json_http_response()],
            &mut cache,
        )
        .unwrap_err();
        assert_eq!(
            error.source_info.start,
            Pos {
                line: 1,
                column: 10,
            }
        );
        assert_eq!(
            error.kind,
            RunnerErrorKind::QueryInvalidJsonpathExpression {
                value: "xxx".to_string()
            }
        );
    }

    #[test]
    fn test_query_invalid_json() {
        let variables = VariableSet::new();
        let mut cache = BodyCache::new();
        let http_response = Response {
            body: String::into_bytes(String::from("xxx")),
            ..default_response()
        };
        let error = eval_query(
            &jsonpath_success(),
            &variables,
            &[&http_response],
            &mut cache,
        )
        .err()
        .unwrap();
        assert_eq!(error.source_info.start, Pos { line: 1, column: 1 });
        assert_eq!(error.kind, RunnerErrorKind::QueryInvalidJson);
    }

    #[test]
    fn test_query_json_not_found() {
        let variables = VariableSet::new();
        let mut cache = BodyCache::new();

        let http_response = Response {
            body: String::into_bytes(String::from("{}")),
            ..default_response()
        };
        assert_eq!(
            eval_query(
                &jsonpath_success(),
                &variables,
                &[&http_response],
                &mut cache
            )
            .unwrap(),
            None
        );
    }

    #[test]
    fn test_query_json() {
        let variables = VariableSet::new();
        let mut cache = BodyCache::new();

        assert_eq!(
            eval_query(
                &jsonpath_success(),
                &variables,
                &[&http::json_http_response()],
                &mut cache
            )
            .unwrap()
            .unwrap(),
            Value::Bool(false)
        );
        assert_eq!(
            eval_query(
                &jsonpath_errors(),
                &variables,
                &[&http::json_http_response()],
                &mut cache
            )
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
        let variables = VariableSet::new();
        let mut cache = BodyCache::new();

        assert_eq!(
            eval_query(
                &regex_name(),
                &variables,
                &[&http::hello_http_response()],
                &mut cache
            )
            .unwrap()
            .unwrap(),
            Value::String("World".to_string())
        );

        let error = eval_query(
            &regex_invalid(),
            &variables,
            &[&http::hello_http_response()],
            &mut cache,
        )
        .err()
        .unwrap();
        assert_eq!(
            error.source_info,
            SourceInfo::new(Pos::new(1, 7), Pos::new(1, 10))
        );
        assert_eq!(error.kind, RunnerErrorKind::InvalidRegex);
    }

    #[test]
    fn test_query_bytes() {
        let variables = VariableSet::new();
        let mut cache = BodyCache::new();

        assert_eq!(
            eval_query(
                &Query {
                    source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                    value: QueryValue::Bytes,
                },
                &variables,
                &[&http::hello_http_response()],
                &mut cache,
            )
            .unwrap()
            .unwrap(),
            Value::Bytes(String::into_bytes(String::from("Hello World!")))
        );
    }

    fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
            .collect()
    }

    #[test]
    fn test_query_sha256() {
        let variables = VariableSet::new();
        let mut cache = BodyCache::new();

        assert_eq!(
            eval_query(
                &Query {
                    source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                    value: QueryValue::Sha256 {},
                },
                &variables,
                &[&Response {
                    body: vec![0xff],
                    ..default_response()
                }],
                &mut cache,
            )
            .unwrap()
            .unwrap(),
            Value::Bytes(
                decode_hex("a8100ae6aa1940d0b663bb31cd466142ebbdbd5187131b92d93818987832eb89")
                    .unwrap()
            )
        );
    }

    #[test]
    fn test_query_certificate() {
        assert!(eval_query_certificate(
            &Response {
                ..default_response()
            },
            CertificateAttributeName::Subject
        )
        .unwrap()
        .is_none());
        assert_eq!(
            eval_query_certificate(
                &Response {
                    certificate: Some(http::Certificate {
                        subject: "A=B, C=D".to_string(),
                        issuer: String::new(),
                        start_date: Default::default(),
                        expire_date: Default::default(),
                        serial_number: String::new()
                    }),
                    ..default_response()
                },
                CertificateAttributeName::Subject
            )
            .unwrap()
            .unwrap(),
            Value::String("A=B, C=D".to_string())
        );
    }
}
