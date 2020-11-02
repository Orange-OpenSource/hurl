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

use core::fmt;

use super::core::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Request {
    pub method: Method,
    pub url: String,
    pub headers: Vec<Header>,
    pub querystring: Vec<Param>,
    pub form: Vec<Param>,
    pub multipart: Vec<MultipartParam>,
    pub cookies: Vec<RequestCookie>,
    pub body: Vec<u8>,
    pub content_type: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Method {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Patch,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Param {
    pub name: String,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MultipartParam {
    Param(Param),
    FileParam(FileParam),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FileParam {
    pub name: String,
    pub filename: String,
    pub data: Vec<u8>,
    pub content_type: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RequestCookie {
    pub name: String,
    pub value: String,
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Method::Get => "GET",
            Method::Head => "HEAD",
            Method::Post => "POST",
            Method::Put => "PUT",
            Method::Delete => "DELETE",
            Method::Connect => "CONNECT",
            Method::Options => "OPTIONS",
            Method::Trace => "TRACE",
            Method::Patch => "PATCH",
        };
        write!(f, "{}", value)
    }
}

impl fmt::Display for Param {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.value)
    }
}

impl fmt::Display for MultipartParam {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MultipartParam::Param(param) => write!(f, "{}", param.to_string()),
            MultipartParam::FileParam(param) => write!(f, "{}", param.to_string()),
        }
    }
}

impl fmt::Display for FileParam {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}: file,{}; {}",
            self.name, self.filename, self.content_type
        )
    }
}

impl fmt::Display for RequestCookie {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}={}", self.name, self.value)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn hello_http_request() -> Request {
        Request {
            method: Method::Get,
            url: "http://localhost:8000/hello".to_string(),
            querystring: vec![],
            headers: vec![],
            cookies: vec![],
            body: vec![],
            multipart: vec![],
            form: vec![],
            content_type: None,
        }
    }

    pub fn custom_http_request() -> Request {
        Request {
            method: Method::Get,
            url: "http://localhost/custom".to_string(),
            querystring: vec![],
            headers: vec![
                Header {
                    name: String::from("User-Agent"),
                    value: String::from("iPhone"),
                },
                Header {
                    name: String::from("Foo"),
                    value: String::from("Bar"),
                },
            ],
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
            body: vec![],
            multipart: vec![],
            form: vec![],
            content_type: None,
        }
    }

    pub fn query_http_request() -> Request {
        Request {
            method: Method::Get,
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
            headers: vec![],
            cookies: vec![],
            body: vec![],
            multipart: vec![],
            form: vec![],
            content_type: None,
        }
    }

    pub fn form_http_request() -> Request {
        Request {
            method: Method::Post,
            url: "http://localhost/form-params".to_string(),
            querystring: vec![],
            headers: vec![Header {
                name: String::from("Content-Type"),
                value: String::from("application/x-www-form-urlencoded"),
            }],
            cookies: vec![],
            body: "param1=value1&param2=&param3=a%3db&param4=a%253db"
                .to_string()
                .into_bytes(),
            multipart: vec![],
            form: vec![],
            content_type: Some("multipart/form-data".to_string()),
        }
    }
}
