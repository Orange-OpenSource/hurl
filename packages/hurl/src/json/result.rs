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
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};

use chrono::SecondsFormat;
use hurl_core::ast::SourceInfo;
use hurl_core::error::{DisplaySourceError, OutputFormat};
use hurl_core::input::Input;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::http::{
    Call, Certificate, Cookie, Header, HttpVersion, Param, Request, RequestCookie, Response,
    ResponseCookie, Timings,
};
use crate::runner::{AssertResult, CaptureResult, EntryResult, HurlResult};
use crate::util::redacted::Redact;

impl HurlResult {
    /// Serializes an [`HurlResult`] to a JSON representation.
    ///
    /// Note: `content` is passed to this method to save asserts and errors messages (with lines
    /// and columns). This parameter will be removed soon and the original content will be
    /// accessible through the [`HurlResult`] instance.
    /// An optional directory `response_dir` can be used to save HTTP response.
    /// `secrets` strings are redacted from the JSON fields.
    pub fn to_json(
        &self,
        content: &str,
        filename: &Input,
        response_dir: Option<&Path>,
        secrets: &[&str],
    ) -> Result<serde_json::Value, io::Error> {
        let result = HurlResultJson::from_result(self, content, filename, response_dir, secrets)?;
        let value = serde_json::to_value(result)?;
        Ok(value)
    }

    /// Checks if a JSON value can be deserialized to a `HurlResult` instance.
    /// This method can be used to check if the schema of the `value` is conform to
    /// a `HurlResult`.
    pub fn is_deserializable(value: &serde_json::Value) -> bool {
        serde_json::from_value::<HurlResultJson>(value.clone()).is_ok()
    }
}

/// These structures represent the JSON schema used to serialize an [`HurlResult`] to JSON.
#[derive(Deserialize, Serialize)]
struct HurlResultJson {
    filename: String,
    entries: Vec<EntryResultJson>,
    success: bool,
    time: u64,
    cookies: Vec<CookieJson>,
}

#[derive(Deserialize, Serialize)]
struct EntryResultJson {
    index: usize,
    line: usize,
    calls: Vec<CallJson>,
    captures: Vec<CaptureJson>,
    asserts: Vec<AssertJson>,
    time: u64,
    curl_cmd: String,
}

#[derive(Deserialize, Serialize)]
struct CookieJson {
    domain: String,
    include_subdomain: String,
    path: String,
    https: String,
    expires: String,
    name: String,
    value: String,
}

#[derive(Deserialize, Serialize)]
struct CallJson {
    request: RequestJson,
    response: ResponseJson,
    timings: TimingsJson,
}

#[derive(Deserialize, Serialize)]
struct CaptureJson {
    name: String,
    value: serde_json::Value,
}

#[derive(Deserialize, Serialize)]
struct AssertJson {
    success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
    line: usize,
}

#[derive(Deserialize, Serialize)]
struct RequestJson {
    method: String,
    url: String,
    headers: Vec<HeaderJson>,
    cookies: Vec<RequestCookieJson>,
    query_string: Vec<ParamJson>,
}

#[derive(Deserialize, Serialize)]
struct ResponseJson {
    http_version: String,
    status: u32,
    headers: Vec<HeaderJson>,
    cookies: Vec<ResponseCookieJson>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate: Option<CertificateJson>,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<String>,
}

#[derive(Deserialize, Serialize)]
struct TimingsJson {
    begin_call: String,
    end_call: String,
    name_lookup: u64,
    connect: u64,
    app_connect: u64,
    pre_transfer: u64,
    start_transfer: u64,
    total: u64,
}

#[derive(Deserialize, Serialize)]
struct HeaderJson {
    name: String,
    value: String,
}

#[derive(Deserialize, Serialize)]
struct RequestCookieJson {
    name: String,
    value: String,
}

#[derive(Deserialize, Serialize)]
struct ParamJson {
    name: String,
    value: String,
}

#[derive(Deserialize, Serialize)]
struct ResponseCookieJson {
    name: String,
    value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires: Option<String>,
    // FIXME: maybe max_age should be u64
    #[serde(skip_serializing_if = "Option::is_none")]
    max_age: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secure: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "httponly")]
    http_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "same_site")]
    same_site: Option<String>,
}

#[derive(Deserialize, Serialize)]
struct CertificateJson {
    subject: String,
    issuer: String,
    start_date: String,
    expire_date: String,
    serial_number: String,
}

impl HurlResultJson {
    fn from_result(
        result: &HurlResult,
        content: &str,
        filename: &Input,
        response_dir: Option<&Path>,
        secrets: &[&str],
    ) -> Result<Self, io::Error> {
        let entries = result
            .entries
            .iter()
            .map(|e| EntryResultJson::from_entry(e, content, filename, response_dir, secrets))
            .collect::<Result<Vec<_>, _>>()?;
        let cookies = result
            .cookies
            .iter()
            .map(|c| CookieJson::from_cookie(c, secrets))
            .collect::<Vec<_>>();
        Ok(HurlResultJson {
            filename: filename.to_string(),
            entries,
            success: result.success,
            time: result.duration.as_millis() as u64,
            cookies,
        })
    }
}

impl EntryResultJson {
    fn from_entry(
        entry: &EntryResult,
        content: &str,
        filename: &Input,
        response_dir: Option<&Path>,
        secrets: &[&str],
    ) -> Result<Self, io::Error> {
        let calls = entry
            .calls
            .iter()
            .map(|c| CallJson::from_call(c, response_dir, secrets))
            .collect::<Result<Vec<_>, _>>()?;
        let captures = entry
            .captures
            .iter()
            .map(|c| CaptureJson::from_capture(c, secrets))
            .collect::<Vec<_>>();
        let asserts = entry
            .asserts
            .iter()
            .map(|a| AssertJson::from_assert(a, content, filename, entry.source_info, secrets))
            .collect::<Vec<_>>();
        Ok(EntryResultJson {
            index: entry.entry_index,
            line: entry.source_info.start.line,
            calls,
            captures,
            asserts,
            time: entry.transfer_duration.as_millis() as u64,
            curl_cmd: entry.curl_cmd.to_string().redact(secrets),
        })
    }
}

impl CookieJson {
    fn from_cookie(c: &Cookie, secrets: &[&str]) -> Self {
        CookieJson {
            domain: c.domain.clone(),
            include_subdomain: c.include_subdomain.clone(),
            path: c.path.clone(),
            https: c.https.clone(),
            expires: c.expires.clone(),
            name: c.name.clone(),
            value: c.value.redact(secrets),
        }
    }
}

impl CallJson {
    fn from_call(
        call: &Call,
        response_dir: Option<&Path>,
        secrets: &[&str],
    ) -> Result<Self, io::Error> {
        let request = RequestJson::from_request(&call.request, secrets);
        let response = ResponseJson::from_response(&call.response, response_dir, secrets)?;
        let timings = TimingsJson::from_timings(&call.timings);
        Ok(CallJson {
            request,
            response,
            timings,
        })
    }
}

impl RequestJson {
    fn from_request(request: &Request, secrets: &[&str]) -> Self {
        let headers = request
            .headers
            .iter()
            .map(|h| HeaderJson::from_header(h, secrets))
            .collect::<Vec<_>>();
        let cookies = request
            .cookies()
            .iter()
            .map(|c| RequestCookieJson::from_cookie(c, secrets))
            .collect::<Vec<_>>();
        let query_string = request
            .url
            .query_params()
            .iter()
            .map(|p| ParamJson::from_param(p, secrets))
            .collect::<Vec<_>>();
        RequestJson {
            method: request.method.clone(),
            url: request.url.to_string().redact(secrets),
            headers,
            cookies,
            query_string,
        }
    }
}

impl ResponseJson {
    fn from_response(
        response: &Response,
        response_dir: Option<&Path>,
        secrets: &[&str],
    ) -> Result<Self, io::Error> {
        let http_version = match response.version {
            HttpVersion::Http10 => "HTTP/1.0",
            HttpVersion::Http11 => "HTTP/1.1",
            HttpVersion::Http2 => "HTTP/2",
            HttpVersion::Http3 => "HTTP/3",
        };
        let headers = response
            .headers
            .iter()
            .map(|h| HeaderJson::from_header(h, secrets))
            .collect::<Vec<_>>();
        let cookies = response
            .cookies()
            .iter()
            .map(|c| ResponseCookieJson::from_cookie(c, secrets))
            .collect::<Vec<_>>();
        let certificate = response
            .certificate
            .as_ref()
            .map(CertificateJson::from_certificate);
        let body = match response_dir {
            Some(response_dir) => {
                // FIXME: we save the filename and the parent dir: this feature is used in the
                // context of the JSON report where the response are stored:
                //
                // ```
                // response_dir
                // ├── report.json
                // └── store
                //     ├── 1fe9d647-5689-4130-b4ea-dc120c2536ba_response.html
                //     ├── 35f49c69-15f9-43df-a672-a1ff5f68c935_response.json
                //     ...
                //     └── ce7f1326-2e2a-46e9-befd-ee0d85084814_response.json
                // ```
                // we want the `body` field to reference the relative path of a response compared
                // to `report.json`.
                let file = write_response(response, response_dir)?;
                let parent = response_dir.components().last().unwrap();
                let parent: &Path = parent.as_ref();
                Some(format!("{}/{}", parent.display(), file.display()))
            }
            None => None,
        };
        Ok(ResponseJson {
            http_version: http_version.to_string(),
            status: response.status,
            headers,
            cookies,
            certificate,
            body,
        })
    }
}

impl TimingsJson {
    fn from_timings(timings: &Timings) -> Self {
        TimingsJson {
            begin_call: timings
                .begin_call
                .to_rfc3339_opts(SecondsFormat::Micros, true),
            end_call: timings
                .end_call
                .to_rfc3339_opts(SecondsFormat::Micros, true),
            name_lookup: timings.name_lookup.as_micros() as u64,
            connect: timings.connect.as_micros() as u64,
            app_connect: timings.app_connect.as_micros() as u64,
            pre_transfer: timings.pre_transfer.as_micros() as u64,
            start_transfer: timings.start_transfer.as_micros() as u64,
            total: timings.total.as_micros() as u64,
        }
    }
}

impl HeaderJson {
    fn from_header(h: &Header, secrets: &[&str]) -> Self {
        HeaderJson {
            name: h.name.clone(),
            value: h.value.redact(secrets),
        }
    }
}

impl RequestCookieJson {
    fn from_cookie(c: &RequestCookie, secrets: &[&str]) -> Self {
        RequestCookieJson {
            name: c.name.clone(),
            value: c.value.redact(secrets),
        }
    }
}

impl ParamJson {
    fn from_param(p: &Param, secrets: &[&str]) -> Self {
        ParamJson {
            name: p.name.clone(),
            value: p.value.redact(secrets),
        }
    }
}

impl ResponseCookieJson {
    fn from_cookie(c: &ResponseCookie, secrets: &[&str]) -> Self {
        ResponseCookieJson {
            name: c.name.clone(),
            value: c.value.redact(secrets),
            expires: c.expires(),
            max_age: c.max_age().map(|m| m.to_string()),
            domain: c.domain(),
            path: c.path(),
            secure: if c.has_secure() { Some(true) } else { None },
            http_only: if c.has_httponly() { Some(true) } else { None },
            same_site: c.samesite(),
        }
    }
}

impl CertificateJson {
    fn from_certificate(c: &Certificate) -> Self {
        CertificateJson {
            subject: c.subject.clone(),
            issuer: c.issuer.clone(),
            start_date: c.start_date.to_string(),
            expire_date: c.expire_date.to_string(),
            serial_number: c.serial_number.to_string(),
        }
    }
}

impl CaptureJson {
    fn from_capture(c: &CaptureResult, secrets: &[&str]) -> Self {
        CaptureJson {
            name: c.name.clone(),
            value: c.value.to_json(secrets),
        }
    }
}

impl AssertJson {
    fn from_assert(
        a: &AssertResult,
        content: &str,
        filename: &Input,
        entry_src_info: SourceInfo,
        secrets: &[&str],
    ) -> Self {
        let message = a.error().map(|err| {
            err.to_string(
                &filename.to_string(),
                content,
                Some(entry_src_info),
                OutputFormat::Plain,
            )
        });
        let message = message.map(|m| m.redact(secrets));
        AssertJson {
            success: a.error().is_none(),
            message,
            line: a.line(),
        }
    }
}

/// Write the HTTP `response` body to directory `dir`.
fn write_response(response: &Response, dir: &Path) -> Result<PathBuf, io::Error> {
    let extension = if response.is_json() {
        Some("json")
    } else if response.is_xml() {
        Some("xml")
    } else if response.is_html() {
        Some("html")
    } else {
        None
    };
    let id = Uuid::new_v4();
    let relative_path = format!("{id}_response");
    let relative_path = Path::new(&relative_path);
    let relative_path = match extension {
        Some(ext) => relative_path.with_extension(ext),
        None => relative_path.to_path_buf(),
    };
    let path = dir.join(relative_path.clone());
    let mut file = File::create(path)?;
    file.write_all(&response.body)?;
    Ok(relative_path)
}
