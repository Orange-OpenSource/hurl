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


use crate::http::*;
use super::cookie::*;
use super::core::*;

type ParseError = String;


pub fn parse_results(value: serde_json::Value) -> Result<Vec<HurlResult>, ParseError> {
    if let serde_json::Value::Array(values) = value {
        let mut results = vec![];
        for value in values {
            let result = parse_result(value)?;
            results.push(result);
        }
        Ok(results)
    } else {
        Err("expecting an array of session".to_string())
    }
}


fn parse_result(value: serde_json::Value) -> Result<HurlResult, ParseError> {
    if let serde_json::Value::Object(map) = value.clone() {
        let filename = map.get("filename").unwrap().as_str().unwrap().to_string();
        let mut entries = vec![];
        let entries = if let Some(serde_json::Value::Array(values)) = map.get("entries") {
            for value in values {
                let entry = parse_entry_result(value.clone())?;
                entries.push(entry);
            }
            entries
        } else {
            return Err("expecting an array of entries".to_string());
        };
        let time_in_ms = match value.get("time") {
            Some(serde_json::Value::Number(n)) => {
                match n.as_u64() {
                    Some(x) => x as u128,
                    None => return Err("expecting an integer for the time".to_string()),
                }
            }
            _ => return Err("expecting an integer for the time".to_string()),
        };
        let success = match value.get("success") {
            Some(serde_json::Value::Bool(v)) => *v,
            _ => return Err("expecting a bool for the status".to_string()),
        };
        let cookies = vec![];
        Ok(HurlResult { filename, entries, time_in_ms, success, cookies })
    } else {
        Err("expecting an object for the result".to_string())
    }
}

fn parse_entry_result(value: serde_json::Value) -> Result<EntryResult, String> {
    let request = match value.get("request") {
        None => None,
        Some(v) => {
            let r = parse_request(v.clone())?;
            Some(r)
        }
    };
    let response = match value.get("response") {
        None => None,
        Some(v) => {
            let r = parse_response(v.clone())?;
            Some(r)
        }
    };
    Ok(EntryResult {
        request,
        response,
        captures: vec![],
        asserts: vec![],
        errors: vec![],
        time_in_ms: 0,
    })
}


pub fn parse_request(value: serde_json::Value) -> Result<Request, ParseError> {
    if let serde_json::Value::Object(map) = value {
        let method = match map.get("method") {
            Some(serde_json::Value::String(s)) => parse_method(s.clone())?,
            _ => return Err("expecting a string for the method".to_string()),
        };
        let url = match map.get("url") {
            Some(serde_json::Value::String(s)) => s.to_string(),
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

        let querystring = match map.get("queryString") {
            Some(serde_json::Value::Array(values)) => {
                let mut params = vec![];
                for value in values {
                    let param = parse_param(value.clone())?;
                    params.push(param);
                }
                params
            }
            _ => vec![],
        };

        let form = match map.get("form") {
            Some(serde_json::Value::Array(values)) => {
                let mut params = vec![];
                for value in values {
                    let param = parse_param(value.clone())?;
                    params.push(param);
                }
                params
            }
            _ => vec![],
        };


        let cookies = match map.get("cookies") {
            Some(serde_json::Value::Array(values)) => {
                let mut headers = vec![];
                for value in values {
                    let header = parse_request_cookie(value.clone())?;
                    headers.push(header);
                }
                headers
            }
            _ => vec![],
        };

        // TODO
        let multipart = vec![];
        let body = vec![];
        let content_type = None;

        Ok(Request {
            method,
            url,
            querystring,
            headers,
            cookies,
            body,
            multipart,
            form,
            content_type,
        })
    } else {
        Err("expecting an object for the request".to_string())
    }
}

pub fn parse_response(value: serde_json::Value) -> Result<Response, ParseError> {
    if let serde_json::Value::Object(map) = value {
        let status = match map.get("status") {
            Some(serde_json::Value::Number(x)) => if let Some(x) = x.as_u64() {
                x as u32
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

pub fn parse_param(value: serde_json::Value) -> Result<Param, ParseError> {
    if let serde_json::Value::Object(map) = value {
        let name = match map.get("name") {
            Some(serde_json::Value::String(s)) => s.to_string(),
            _ => return Err("expecting a string for the cookie name".to_string()),
        };
        let value = match map.get("value") {
            Some(serde_json::Value::String(s)) => s.to_string(),
            _ => return Err("expecting a string for the cookie value".to_string()),
        };
        Ok(Param { name, value })
    } else {
        Err("Expecting object for the param".to_string())
    }
}

pub fn parse_request_cookie(value: serde_json::Value) -> Result<RequestCookie, ParseError> {
    if let serde_json::Value::Object(map) = value {
        let name = match map.get("name") {
            Some(serde_json::Value::String(s)) => s.to_string(),
            _ => return Err("expecting a string for the cookie name".to_string()),
        };
        let value = match map.get("value") {
            Some(serde_json::Value::String(s)) => s.to_string(),
            _ => return Err("expecting a string for the cookie value".to_string()),
        };
        Ok(RequestCookie { name, value })
    } else {
        Err("Expecting object for the request cookie".to_string())
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
        let mut attributes = vec![];

        match map.get("expires") {
            None => {}
            Some(serde_json::Value::String(s)) => attributes.push(CookieAttribute { name: "Expires".to_string(), value: Some(s.to_string()) }),
            _ => return Err("expecting a string for the cookie expires".to_string()),
        };
        match map.get("max_age") {
            None => {}
            Some(serde_json::Value::Number(n)) => attributes.push(CookieAttribute { name: "Max-Age".to_string(), value: Some(n.to_string()) }),
            _ => return Err("expecting an integer for the cookie max_age".to_string()),
        };
        match map.get("domain") {
            None => {}
            Some(serde_json::Value::String(s)) => attributes.push(CookieAttribute { name: "Domain".to_string(), value: Some(s.to_string()) }),
            _ => return Err("expecting a string for the cookie domain".to_string()),
        };
        match map.get("path") {
            None => {}
            Some(serde_json::Value::String(s)) => attributes.push(CookieAttribute { name: "Path".to_string(), value: Some(s.to_string()) }),
            _ => return Err("expecting a string for the cookie path".to_string()),
        };

        match map.get("secure") {
            None => {}
            Some(serde_json::Value::Bool(true)) => attributes.push(CookieAttribute { name: "Secure".to_string(), value: None }),
            _ => return Err("expecting a true for the cookie secure flag".to_string()),
        };
        match map.get("http_only") {
            None => {}
            Some(serde_json::Value::Bool(true)) => attributes.push(CookieAttribute { name: "HttpOnly".to_string(), value: None }),
            _ => return Err("expecting a true for the cookie http_only flag".to_string()),
        };
        match map.get("same_site") {
            None => {}
            Some(serde_json::Value::String(s)) => attributes.push(CookieAttribute { name: "SameSite".to_string(), value: Some(s.to_string()) }),
            _ => return Err("expecting a string for the cookie same_site".to_string()),
        };

        Ok(ResponseCookie { name, value, attributes })
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
            url: "http://localhost:8000/querystring-params?param1=value1&param2=a%20b".to_string(),
            querystring: vec![],
            headers: vec![],
            cookies: vec![],
            body: vec![],
            form: vec![],
            multipart: vec![],
            content_type: None,
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
                       attributes: vec![],
                   }
        );
    }

    #[test]
    fn test_parse_version() {
        assert_eq!(parse_version("HTTP/1.0".to_string()).unwrap(), Version::Http10);
    }
}
