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
use core::fmt;

use crate::http::header::HeaderVec;
use crate::http::{Param, RequestCookie, Url};

/// Represents the HTTP request asked to be executed by our user (different from the runtime
/// executed HTTP request [`crate::http::Request`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RequestSpec {
    pub method: Method,
    pub url: Url,
    pub headers: HeaderVec,
    pub querystring: Vec<Param>,
    pub form: Vec<Param>,
    pub multipart: Vec<MultipartParam>,
    pub cookies: Vec<RequestCookie>,
    pub body: Body,
    /// This is the implicit content type of the request: this content type is implicitly set when
    /// the request use a "typed" body: form, JSON, multipart, multiline string with hint. This
    /// implicit content type can be different from the user provided one through the `headers`
    /// field.
    pub implicit_content_type: Option<String>,
}

impl Default for RequestSpec {
    fn default() -> Self {
        RequestSpec {
            method: Method("GET".to_string()),
            url: Url::default(),
            headers: HeaderVec::new(),
            querystring: vec![],
            form: vec![],
            multipart: vec![],
            cookies: vec![],
            body: Body::Binary(vec![]),
            implicit_content_type: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Method(pub String);

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
pub enum Body {
    Text(String),
    Binary(Vec<u8>),
    File(Vec<u8>, String),
}

impl Body {
    pub fn bytes(&self) -> Vec<u8> {
        match self {
            Body::Text(s) => s.as_bytes().to_vec(),
            Body::Binary(bs) => bs.clone(),
            Body::File(bs, _) => bs.clone(),
        }
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for MultipartParam {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MultipartParam::Param(param) => write!(f, "{param}"),
            MultipartParam::FileParam(param) => write!(f, "{param}"),
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
