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
use std::time::Instant;

use encoding::{DecoderTrap, Encoding};
use encoding::all::ISO_8859_1;

use crate::core::ast::*;
use crate::core::common::SourceInfo;
use crate::core::common::Value;
use crate::http;
use crate::http::cookie::CookieJar;

use super::core::*;
use super::core::{Error, RunnerError};
use crate::format::logger::Logger;



/// Run an entry with the hurl http client
///
/// # Examples
///
/// ```
/// use hurl::http;
/// use hurl::runner;
///
/// // Create an http client
/// let client = http::client::Client::init(http::client::ClientOptions {
///        noproxy_hosts: vec![],
///        insecure: false,
///        redirect: http::client::Redirect::None,
///        http_proxy: None,
///        https_proxy: None,
///        all_proxy: None
///    });
/// ```
pub fn run(entry: Entry, http_client: &http::client::Client,
           entry_index: usize,
           variables: &mut HashMap<String, Value>,
           cookiejar: &mut CookieJar,
           context_dir: String,
           logger: &Logger,
) -> EntryResult {

    //let mut entry_log_builder = EntryLogBuilder::init();

    let mut http_request = match entry.clone().request.eval(variables, context_dir.clone()) {
        Ok(r) => r,
        Err(error) => {
            return EntryResult {
                request: None,
                response: None,
                captures: vec![],
                asserts: vec![],
                errors: vec![error],

                time_in_ms: 0,
            };
        }
    };
    let cookies = cookiejar.clone().get_cookies(http_request.clone().host(), String::from("/"));
    http_request.add_session_cookies(cookies);

    logger.verbose("------------------------------------------------------------------------------");
    logger.verbose(format!("executing entry {}", entry_index + 1).as_str());
    for line in http_request.verbose_output() {
        logger.send(line);
    }
    if let Some(params) = http_request.clone().form_params() {
        logger.verbose("Form Params");
        for param in params {
            logger.verbose(format!("   {}={}", param.name, param.value).as_str());
        }
    };

    let start = Instant::now();
    let http_response = match http_client.execute(&http_request) {
        Ok(response) => response,
        Err(e) => {
            return EntryResult {
                request: Some(http_request),
                response: None,
                captures: vec![],
                asserts: vec![],
                errors: vec![
                    Error {
                        source_info: SourceInfo {
                            start: entry.clone().request.url.source_info.start,
                            end: entry.clone().request.url.source_info.end,
                        },
                        inner: RunnerError::HttpConnection {
                            message: e.message,
                            url: e.url,
                        },
                        assert: false,
                    }],
                time_in_ms: 0,
            };
        }
    };
    let time_in_ms = start.elapsed().as_millis();
    for line in http_response.verbose_output() {
        logger.receive(line);
    }


    //entry_log_builder.response(http_response.clone(), verbose);

    //hurl_log.entries.push(log_builder.build());
    let captures = match entry.response.clone() {
        None => vec![],
        Some(response) => match response.eval_captures(http_response.clone(), variables) {
            Ok(captures) => captures,
            Err(e) => {
                return EntryResult {
                    request: Some(http_request),
                    response: Some(http_response),
                    captures: vec![],
                    asserts: vec![],
                    errors: vec![e],
                    time_in_ms,
                };
            }
        }
    };

    // update variables now!
    for capture_result in captures.clone() {
        variables.insert(capture_result.name, capture_result.value);
    }


    let asserts = match entry.response {
        None => vec![],
        Some(response) => response.eval_asserts(variables, http_response.clone(), context_dir)
    };

    let errors = asserts
        .iter()
        .filter_map(|assert| assert.clone().error())
        .map(|Error { source_info, inner, .. }| Error { source_info, inner, assert: true })
        .collect();


    // update cookies
    // for the domain
    let domain = http_request.clone().host();
    //let mut cookies = cookie_store.get_cookies(host);

    // TEMPORARY also update store from request cookie
    // TODO - DO BE REMOVED - add explicit directive in hurl file to interract with cookiejar
    for cookie in http_request.clone().cookies {
        cookiejar.update_cookies(
            domain.clone(),
            http_request.clone().url.path,
            cookie,
        );
    }


    for cookie in http_response.cookies() {
        cookiejar.update_cookies(
            domain.clone(),
            http_request.clone().url.path,
            cookie,
        );
    }


    if !captures.is_empty() {
        logger.verbose("Captures");
        for capture in captures.clone() {
            logger.verbose(format!("{}: {:?}", capture.name, capture.value).as_str());
        }
    }
    if !cookiejar.clone().cookies().is_empty() {
        logger.verbose("CookieJar");
        for cookie in cookiejar.clone().cookies() {
            logger.verbose(cookie.to_string().as_str());
        }
    }

    EntryResult {
        request: Some(http_request),
        response: Some(http_response),
        captures,
        asserts,
        errors,
        time_in_ms,
    }
}




// cookies
// for all domains

// but only pass cookies for one domain for the request

fn decode_bytes(bytes: Vec<u8>, encoding: http::core::Encoding) -> Result<String, RunnerError> {
    match encoding {
        http::core::Encoding::Utf8 {} => match String::from_utf8(bytes) {
            Ok(s) => Ok(s),
            Err(_) => Err(RunnerError::InvalidDecoding { charset: encoding.to_string() }),
        },
        http::core::Encoding::Latin1 {} => match ISO_8859_1.decode(&bytes, DecoderTrap::Strict) {
            Ok(s) => Ok(s),
            Err(_) => Err(RunnerError::InvalidDecoding { charset: encoding.to_string() }),
        },
    }
}


impl http::request::Request {
    pub fn verbose_output(&self) -> Vec<String> {
        let mut lines = vec![];
        lines.push(format!("{} {}", self.clone().method.to_text(), self.clone().url()));
        for header in self.clone().headers() {
            lines.push(header.to_string());
        }
        lines.push("".to_string());
        if !self.body.is_empty() {
            let body = match decode_bytes(self.body.clone(), self.encoding()) {
                Ok(s) => s,
                Err(_) => format!("{:x?}", self.body)
            };
            let body_lines: Vec<&str> = regex::Regex::new(r"\n|\r\n")
                .unwrap()
                .split(&body)
                .collect();
            for line in body_lines {
                lines.push(line.to_string());
            }
        }
        lines
    }
}

impl http::response::Response {
    pub fn verbose_output(&self) -> Vec<String> {
        let mut lines = vec![];
        lines.push(format!("HTTP/{} {}", self.version.to_text(), self.status));
        for header in self.clone().headers {
            lines.push(header.to_string());
        }
        lines.push("".to_string());
        if !self.body.is_empty() {

            //let body = body_text(self.clone().body, get_header_value(self.clone().headers, "content-type"));
            //let body = substring(body.as_str(), 0, limit_body);
            let body = match decode_bytes(self.body.clone(), self.encoding()) {
                Ok(s) => s,
                Err(_) => format!("{:x?}", self.body)
            };
            let body_lines: Vec<&str> = regex::Regex::new(r"\n|\r\n")
                .unwrap()
                .split(&body)
                .collect();
            for line in body_lines {
                lines.push(line.to_string());
            }
        }
        lines
    }
}
