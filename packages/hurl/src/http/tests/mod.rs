/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2023 Orange
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
use crate::http::{Header, HeaderVec, Method, Param, RequestCookie, RequestSpec, Response};

/// Some Request Response to be used by tests

pub fn hello_http_request() -> RequestSpec {
    RequestSpec {
        method: Method("GET".to_string()),
        url: "http://localhost:8000/hello".to_string(),
        ..Default::default()
    }
}

pub fn json_http_response() -> Response {
    Response {
        body: String::into_bytes(
            r#"
{
  "success":false,
  "errors": [
    { "id": "error1"},
    {"id": "error2"}
  ],
  "duration": 1.5
}
"#
            .to_string(),
        ),
        ..Default::default()
    }
}

pub fn xml_two_users_http_response() -> Response {
    let mut headers = HeaderVec::new();
    headers.push(Header::new("Content-Type", "text/html; charset=utf-8"));
    headers.push(Header::new("Content-Length", "12"));

    Response {
        headers,
        body: String::into_bytes(
            r#"
<?xml version="1.0"?>
<users>
  <user id="1">Bob</user>
  <user id="2">Bill</user>
</users>
"#
            .to_string(),
        ),
        ..Default::default()
    }
}

pub fn xml_three_users_http_response() -> Response {
    let mut headers = HeaderVec::new();
    headers.push(Header::new("Content-Type", "text/html; charset=utf-8"));
    headers.push(Header::new("Content-Length", "12"));

    Response {
        headers,
        body: String::into_bytes(
            r#"
<?xml version="1.0"?>
<users>
  <user id="1">Bob</user>
  <user id="2">Bill</user>
  <user id="3">Bruce</user>
</users>
"#
            .to_string(),
        ),
        ..Default::default()
    }
}

pub fn hello_http_response() -> Response {
    let mut headers = HeaderVec::new();
    headers.push(Header::new("Content-Type", "text/html; charset=utf-8"));
    headers.push(Header::new("Content-Length", "12"));

    Response {
        headers,
        body: String::into_bytes(String::from("Hello World!")),
        ..Default::default()
    }
}

pub fn bytes_http_response() -> Response {
    let mut headers = HeaderVec::new();
    headers.push(Header::new("Content-Type", "application/octet-stream"));
    headers.push(Header::new("Content-Length", "1"));

    Response {
        headers,
        body: vec![255],
        ..Default::default()
    }
}

pub fn html_http_response() -> Response {
    let mut headers = HeaderVec::new();
    headers.push(Header::new("Content-Type", "application/octet-stream"));

    Response {
        headers,
        body: String::into_bytes(String::from(
            "<html><head><meta charset=\"UTF-8\"></head><body><br></body></html>",
        )),
        ..Default::default()
    }
}

pub fn query_http_request() -> RequestSpec {
    RequestSpec {
        method: Method("GET".to_string()),
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
        ..Default::default()
    }
}

pub fn custom_http_request() -> RequestSpec {
    let mut headers = HeaderVec::new();
    headers.push(Header::new("User-Agent", "iPhone"));
    headers.push(Header::new("Foo", "Bar"));

    RequestSpec {
        method: Method("GET".to_string()),
        url: "http://localhost/custom".to_string(),
        headers,
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
        ..Default::default()
    }
}
