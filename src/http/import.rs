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
extern crate url as external_url;

use chrono::DateTime;

use super::cookie::*;
use super::core::*;
use super::request::*;
use super::response::*;

type ParseError = String;

pub fn parse_request(value: serde_json::Value) -> Result<Request, ParseError> {
    if let serde_json::Value::Object(map) = value {
        let method = match map.get("method") {
            Some(serde_json::Value::String(s)) => parse_method(s.clone())?,
            _ => return Err("expecting a string for the method".to_string()),
        };
        let url = match map.get("url") {
            Some(serde_json::Value::String(s)) => parse_url(s.clone())?,
            _ => return Err("expecting a string for the url".to_string()),
        };

        let headers = match map.get("headers") {
            Some(serde_json::Value::Array(values)) => {
                let mut headers = vec![];
                for value in values {
                    let header = parse_header(value.clone())?;
                    headers.push(header);
                }
                headers
            }
            _ => vec![],
        };

        let cookies = match map.get("cookies") {
            Some(serde_json::Value::Array(values)) => {
                let mut headers = vec![];
                for value in values {
                    let header = parse_response_cookie(value.clone())?;
                    headers.push(header);
                }
                headers
            }
            _ => vec![],
        };

        let multipart = vec![];

        Ok(Request {
            method,
            url,
            querystring: vec![],
            headers,
            cookies,
            body: vec![],
            multipart,
        })
    } else {
        Err("expecting an object for the request".to_string())
    }
}

pub fn parse_response(value: serde_json::Value) -> Result<Response, ParseError> {
    if let serde_json::Value::Object(map) = value {
        let status = match map.get("status") {
            Some(serde_json::Value::Number(x)) => if let Some(x) = x.as_u64() {
                x as u16
            } else {
                return Err("expecting a integer for the status".to_string());
            },
            _ => return Err("expecting a number for the status".to_string()),
        };

        let version = match map.get("httpVersion") {
            Some(serde_json::Value::String(s)) => parse_version(s.clone())?,
            _ => return Err("expecting a string for the version".to_string()),
        };

        let headers = match map.get("headers") {
            Some(serde_json::Value::Array(values)) => {
                let mut headers = vec![];
                for value in values {
                    let header = parse_header(value.clone())?;
                    headers.push(header);
                }
                headers
            }
            _ => vec![],
        };

        Ok(Response {
            version,
            status,
            headers,
            body: vec![],
        })
    } else {
        Err("expecting an object for the response".to_string())
    }
}

fn parse_method(s: String) -> Result<Method, ParseError> {
    match s.as_str() {
        "GET" => Ok(Method::Get),
        "HEAD" => Ok(Method::Head),
        "POST" => Ok(Method::Post),
        "PUT" => Ok(Method::Put),
        "DELETE" => Ok(Method::Delete),
        "CONNECT" => Ok(Method::Connect),
        "OPTIONS" => Ok(Method::Options),
        "TRACE" => Ok(Method::Trace),
        "PATCH" => Ok(Method::Patch),
        _ => Err(format!("Invalid method <{}>", s))
    }
}

fn parse_url(s: String) -> Result<Url, ParseError> {
    match external_url::Url::parse(s.as_str()) {
        Err(_) => Err(format!("Invalid url <{}>", s)),
        Ok(u) => Ok(Url {
            scheme: u.scheme().to_string(),
            host: u.host_str().unwrap().to_string(),
            port: u.port(),
            path: u.path().to_string(),
            query_string: if let Some(s) = u.query() { s.to_string() } else { "".to_string() },
        })
    }
}

fn parse_header(value: serde_json::Value) -> Result<Header, ParseError> {
    if let serde_json::Value::Object(map) = value {
        let name = match map.get("name") {
            Some(serde_json::Value::String(s)) => s.to_string(),
            _ => return Err("expecting a string for the header name".to_string()),
        };
        let value = match map.get("value") {
            Some(serde_json::Value::String(s)) => s.to_string(),
            _ => return Err("expecting a string for the header value".to_string()),
        };
        Ok(Header { name, value })
    } else {
        Err("Expecting object for one header".to_string())
    }
}

pub fn parse_response_cookie(value: serde_json::Value) -> Result<ResponseCookie, ParseError> {
    if let serde_json::Value::Object(map) = value {
        let name = match map.get("name") {
            Some(serde_json::Value::String(s)) => s.to_string(),
            _ => return Err("expecting a string for the cookie name".to_string()),
        };
        let value = match map.get("value") {
            Some(serde_json::Value::String(s)) => s.to_string(),
            _ => return Err("expecting a string for the cookie value".to_string()),
        };
        let domain = match map.get("domain") {
            None => None,
            Some(serde_json::Value::String(s)) => Some(s.to_string()),
            _ => return Err("expecting a string for the cookie domain".to_string()),
        };
        let path = match map.get("path") {
            None => None,
            Some(serde_json::Value::String(s)) => Some(s.to_string()),
            _ => return Err("expecting a string for the cookie path".to_string()),
        };
        let expires = match map.get("expires") {
            None => None,
            Some(serde_json::Value::String(s)) => Some(s.to_string()),
            _ => return Err("expecting a string for the cookie expires".to_string()),
        };
        let secure = match map.get("secure") {
            None => None,
            Some(serde_json::Value::Bool(value)) => Some(*value),
            _ => return Err("expecting a bool for the cookie secure flag".to_string()),
        };
        let http_only = match map.get("http_only") {
            None => None,
            Some(serde_json::Value::Bool(value)) => Some(*value),
            _ => return Err("expecting a bool for the cookie http_only flag".to_string()),
        };
        let same_site = match map.get("same_site") {
            None => None,
            Some(serde_json::Value::String(s)) => Some(s.to_string()),
            _ => return Err("expecting a string for the cookie same_site".to_string()),
        };
        Ok(ResponseCookie { name, value, max_age: None, domain, path, secure, http_only, expires, same_site })
    } else {
        Err("Expecting object for one cookie".to_string())
    }
}

pub fn parse_cookie(value: serde_json::Value) -> Result<Cookie, ParseError> {
    if let serde_json::Value::Object(map) = value {
        let name = match map.get("name") {
            Some(serde_json::Value::String(s)) => s.to_string(),
            _ => return Err("expecting a string for the cookie name".to_string()),
        };
        let value = match map.get("value") {
            Some(serde_json::Value::String(s)) => s.to_string(),
            _ => return Err("expecting a string for the cookie value".to_string()),
        };
        let domain = match map.get("domain") {
            Some(serde_json::Value::String(s)) => s.to_string(),
            _ => return Err("expecting a string for the cookie domain".to_string()),
        };
        let path = match map.get("path") {
            Some(serde_json::Value::String(s)) => s.to_string(),
            _ => return Err("expecting a string for the cookie path".to_string()),
        };
        let subdomains = match map.get("include_subdomain") {
            Some(serde_json::Value::Bool(v)) => *v,
            _ => return Err("expecting a bool for the include_subdomain".to_string()),
        };
        let secure = match map.get("secure") {
            None => false,
            Some(serde_json::Value::Bool(value)) => *value,
            _ => return Err("expecting a bool for the secure flag".to_string()),
        };
        let expires = match map.get("expired") {
            None => None,
            Some(serde_json::Value::String(v)) => {
                match DateTime::parse_from_rfc3339(v.as_str()) {
                    Ok(v) => Some(v.naive_utc()),
                    Err(_) => return Err("expecting a String (date) for the expired fieldate can be parsed".to_string()),
                }
            }
            _ => return Err("expecting a String (date) for the expired field".to_string()),
        };

        Ok(Cookie { name, value, domain, path, subdomains, secure, expires })
    } else {
        Err("Expecting object for one cookie".to_string())
    }
}

fn parse_version(s: String) -> Result<Version, ParseError> {
    match s.as_str() {
        "HTTP/1.0" => Ok(Version::Http10),
        "HTTP/1.1" => Ok(Version::Http11),
        "HTTP/2" => Ok(Version::Http2),
        _ => Err("Expecting version HTTP/1.0, HTTP/1.2 or HTTP/2".to_string())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::request::tests::*;

    #[test]
    fn test_parse_request() {
        let v: serde_json::Value = serde_json::from_str(r#"{
    "method": "GET",
    "url": "http://localhost:8000/hello",
    "headers": []
}"#).unwrap();
        assert_eq!(parse_request(v).unwrap(), hello_http_request());

        let v: serde_json::Value = serde_json::from_str(r#"{
    "method": "GET",
    "url": "http://localhost:8000/querystring-params?param1=value1&param2=a%20b",
    "headers": []
}"#).unwrap();
        assert_eq!(parse_request(v).unwrap(), Request {
            method: Method::Get,
            url: Url {
                scheme: "http".to_string(),
                host: "localhost".to_string(),
                port: Some(8000),
                path: "/querystring-params".to_string(),
                query_string: "param1=value1&param2=a%20b".to_string(),
            },
            querystring: vec![],
            headers: vec![],
            cookies: vec![],
            body: vec![],
            multipart: vec![],
        });


        let v: serde_json::Value = serde_json::from_str(r#"{
    "method": "GET",
    "url": "http://localhost/custom",
    "headers": [
       {"name": "User-Agent", "value": "iPhone"},
       {"name": "Foo", "value": "Bar"}
    ],
    "cookies": [
       {"name": "theme", "value": "light"},
       {"name": "sessionToken", "value": "abc123"}
    ]
}"#).unwrap();
        assert_eq!(parse_request(v).unwrap(), custom_http_request());
    }

    #[test]
    fn test_parse_response() {
        let v: serde_json::Value = serde_json::from_str(r#"{
    "status": 200,
    "httpVersion": "HTTP/1.0",
    "headers": [
        {"name": "Content-Type", "value": "text/html; charset=utf-8" },
        {"name": "Content-Length", "value": "12" }

    ]
}"#).unwrap();
        assert_eq!(parse_response(v).unwrap(), Response {
            version: Version::Http10,
            status: 200,
            headers: vec![
                Header { name: String::from("Content-Type"), value: String::from("text/html; charset=utf-8") },
                Header { name: String::from("Content-Length"), value: String::from("12") },
            ],
            body: vec![],
        });
    }

    #[test]
    fn test_parse_method() {
        assert_eq!(parse_method("GET".to_string()).unwrap(), Method::Get);

        let error = parse_method("x".to_string()).err().unwrap();
        assert_eq!(error, "Invalid method <x>");
    }

    #[test]
    fn test_parse_url() {
        assert_eq!(
            parse_url("http://localhost:8000/query?param1=value1".to_string()).unwrap(),
            Url {
                scheme: "http".to_string(),
                host: "localhost".to_string(),
                port: Some(8000),
                path: "/query".to_string(),
                query_string: "param1=value1".to_string(),
            });
    }

    #[test]
    fn test_parse_header() {
        let v: serde_json::Value = serde_json::from_str(r#"{
    "name": "name1",
    "value": "value1"
}"#).unwrap();
        assert_eq!(parse_header(v).unwrap(), Header { name: "name1".to_string(), value: "value1".to_string() });
    }

    #[test]
    fn test_parse_response_cookie() {
        let v: serde_json::Value = serde_json::from_str(r#"{
    "name": "name1",
    "value": "value1"
}"#).unwrap();
        assert_eq!(parse_response_cookie(v).unwrap(),
                   ResponseCookie {
                       name: "name1".to_string(),
                       value: "value1".to_string(),
                       max_age: None,
                       domain: None,
                       path: None,
                       secure: None,
                       http_only: None,
                       expires: None,
                       same_site: None,
                   }
        );
    }

    #[test]
    fn test_parse_version() {
        assert_eq!(parse_version("HTTP/1.0".to_string()).unwrap(), Version::Http10);
    }
}
