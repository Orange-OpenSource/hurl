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


use std::path::Path;

use super::core::*;
use super::request::*;
use super::response::*;

pub struct Client {
    _inner_client: reqwest::Client,
    options: ClientOptions,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClientOptions {
    pub noproxy_hosts: Vec<String>,
    pub insecure: bool,
    pub redirect: Redirect,
    pub http_proxy: Option<String>,
    pub https_proxy: Option<String>,
    pub all_proxy: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Redirect {
    None,
    Limited(usize),
    Unlimited,
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HttpError {
    pub url: String,
    pub message: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Proxy {
    pub protocol: Option<ProxyProtocol>,
    pub host: String,
    pub port: Option<u16>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProxyProtocol {
    Http,
    Https,
}


pub fn get_redirect_policy(redirect: Redirect) -> reqwest::RedirectPolicy {
    match redirect {
        Redirect::None => reqwest::RedirectPolicy::none(),
        Redirect::Limited(max) => reqwest::RedirectPolicy::limited(max),
        Redirect::Unlimited {} => reqwest::RedirectPolicy::custom(|attempt| { attempt.follow() }),
    }
}

impl Client {
    pub fn init(options: ClientOptions) -> Client {
        let client_builder = reqwest::Client::builder()
            .redirect(get_redirect_policy(options.redirect.clone()))
            .use_sys_proxy()
            .danger_accept_invalid_hostnames(options.insecure)
            .danger_accept_invalid_certs(options.insecure)
            .cookie_store(false);
        Client {
            _inner_client: client_builder.build().unwrap(),
            options,
        }
    }

    pub fn execute(&self, request: &Request) -> Result<Response, HttpError> {
        let mut headers = reqwest::header::HeaderMap::new();
        for header in request.clone().headers() {
            headers.append(
                reqwest::header::HeaderName::from_lowercase(
                    header.name.to_lowercase().as_str().as_bytes(),
                )
                    .unwrap(),
                reqwest::header::HeaderValue::from_str(header.value.as_str()).unwrap(),
            );
        }


        let client_builder = reqwest::Client::builder()
            .redirect(get_redirect_policy(self.options.redirect.clone()))
            .danger_accept_invalid_hostnames(self.options.insecure)
            .danger_accept_invalid_certs(self.options.insecure)
            .cookie_store(false);


        let client_builder = if let Some(url) = self.options.http_proxy.clone() {
            let proxy = reqwest::Proxy::http(url.as_str()).unwrap();
            client_builder.proxy(proxy)
        } else {
            client_builder
        };
        let client_builder = if let Some(url) = self.options.https_proxy.clone() {
            let proxy = reqwest::Proxy::https(url.as_str()).unwrap();
            client_builder.proxy(proxy)
        } else {
            client_builder
        };
        let client_builder = if let Some(url) = self.options.all_proxy.clone() {
            let proxy = reqwest::Proxy::all(url.as_str()).unwrap();
            client_builder.proxy(proxy)
        } else {
            client_builder
        };

        let client_builder = if self.options.noproxy_hosts.contains(&request.url.host.clone()) {
            client_builder.no_proxy()
        } else {
            client_builder
        };

        let client = client_builder.build().unwrap();


        let req = if request.multipart.is_empty() {
            client
                .request(
                    request.clone().method.to_reqwest(),
                    reqwest::Url::parse(request.clone().url().as_str()).unwrap(),
                )
                .headers(headers)
                .body(request.clone().body)
                .build()
                .unwrap()
        } else {
            let mut form = reqwest::multipart::Form::new();
            for param in request.multipart.clone() {
                match param {
                    MultipartParam::TextParam { name, value } => {
                        form = form.text(name, value)
                    }
                    MultipartParam::FileParam { name, filename, content_type } => {
                        if let Some(content_type) = content_type {
                            let path = Path::new(filename.as_str());
                            let part = reqwest::multipart::Part::file(path).unwrap()
                                .mime_str(content_type.as_str())
                                .unwrap();
                            form = form.part(name, part);
                        } else {
                            form = form.file(name, filename).unwrap();
                        }
                    }
                }
            }
            client
                .request(
                    request.clone().method.to_reqwest(),
                    reqwest::Url::parse(request.clone().url().as_str()).unwrap(),
                )
                .headers(headers)
                .multipart(form)
                .build()
                .unwrap()
        };

        match client.execute(req) {
            Ok(mut resp) => {
                let mut headers = vec![];
                //eprintln!(">>> response headers {:?}", resp.headers().clone());
                for (name, value) in resp.headers() {
                    headers.push(Header {
                        name: name.as_str().to_string(),
                        value: value.to_str().unwrap().to_string(),
                    })
                }

                let version = match resp.version() {
                    reqwest::Version::HTTP_10 => Version::Http10,
                    reqwest::Version::HTTP_11 => Version::Http11,
                    reqwest::Version::HTTP_2 => Version::Http2,
                    v => panic!("Version {:?} not supported!", v),
                };
                let mut buf: Vec<u8> = vec![];
                resp.copy_to(&mut buf).unwrap();    // TODO Test error case
                resp.content_length(); // dirty hack to prevent error "connection closed before message completed"?

                Ok(Response {
                    version,
                    status: resp.status().as_u16(),
                    headers,
                    body: buf,
                })
            }
            Err(e) => {
                Err(HttpError {
                    message: format!("{:?}", e.to_string()),
                    url: request.clone().url(),
                })
            }
        }
    }
}
