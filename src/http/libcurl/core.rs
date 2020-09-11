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
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Response {
    pub status: u32,
    pub headers: Vec<Header>,
    pub body: Vec<u8>,
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
pub struct Header {
    pub name: String,
    pub value: String,
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cookie {
    pub domain: String,
    pub include_subdomain: String,
    pub path: String,
    pub https: String,
    pub expires: String,
    pub name: String,
    pub value: String,
}





impl fmt::Display for RequestCookie {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,  "{}={}", self.name, self.value)
    }
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HttpError {
    CouldNotResolveProxyName,
    CouldNotResolveHost,
    FailToConnect,
    TooManyRedirect
}


impl Response {

    ///
    /// return a list of headers values for the given header name
    ///
    pub fn get_header_values(&self, expected_name: String) -> Vec<String> {
        self.headers
            .iter()
            .filter_map(|Header{ name, value}| if name.clone() == expected_name { Some(value.to_string())} else { None })
            .collect()
    }

}


///
/// return a list of headers values for the given header name
///
pub fn get_header_values(headers: Vec<Header>, expected_name: String) -> Vec<String> {
    headers
        .iter()
        .filter_map(|Header{ name, value}| if name.clone() == expected_name { Some(value.to_string())} else { None })
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn get_header_values() {
        let response = Response {
            status: 200,
            headers: vec![
                Header { name: "Content-Length".to_string(), value: "12".to_string() }
            ],
            body: vec![]
        };
        assert_eq!(response.get_header_values("Content-Length".to_string()), vec!["12".to_string()]);
        assert!(response.get_header_values("Unknown".to_string()).is_empty());

    }

}