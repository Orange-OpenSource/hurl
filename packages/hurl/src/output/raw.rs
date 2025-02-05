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
use std::cmp::min;
use std::io::IsTerminal;

use crate::output::error::OutputErrorKind;
use crate::output::OutputError;
use crate::runner::{HurlResult, Output};
use crate::util::term::Stdout;

/// Writes the `hurl_result` last response to the file `filename_out`.
///
/// If `filename_out` is `None`, standard output is used. If `append` is true, any existing file will
/// be appended instead of being truncated. If `include_headers` is true, the last
/// HTTP response headers are written before the body response.
pub fn write_last_body(
    hurl_result: &HurlResult,
    include_headers: bool,
    color: bool,
    filename_out: Option<&Output>,
    stdout: &mut Stdout,
    append: bool,
) -> Result<(), OutputError> {
    // Get the last call of the Hurl result.
    let Some(last_entry) = &hurl_result.entries.last() else {
        return Ok(());
    };
    let Some(call) = &last_entry.calls.last() else {
        return Ok(());
    };
    let response = &call.response;
    let mut output = vec![];

    // If include options is set, we output the HTTP response headers
    // with status and version (to mimic curl outputs)
    if include_headers {
        let mut text = response.get_status_line_headers(color);
        text.push('\n');
        output.append(&mut text.into_bytes());
    }
    if last_entry.compressed {
        let mut bytes = match response.uncompress_body() {
            Ok(b) => b,
            Err(e) => {
                let source_info = last_entry.source_info;
                let kind = OutputErrorKind::Http(e);
                return Err(OutputError::new(source_info, kind));
            }
        };
        output.append(&mut bytes);
    } else {
        let bytes = &response.body;
        output.extend(bytes);
    }
    // We replicate curl's checks for binary output: a warning is displayed when user hasn't
    // used `--output` option and the response is considered as a binary content. If user has used
    // `--output` whether to save to a file, or to redirect output to standard output (`--output -`)
    // we don't display any warning.
    match filename_out {
        None => {
            if std::io::stdout().is_terminal() && is_binary(&output) {
                let source_info = last_entry.source_info;
                let kind = OutputErrorKind::Binary;
                return Err(OutputError::new(source_info, kind));
            }
            Output::Stdout.write(&output, stdout, append).map_err(|e| {
                let source_info = last_entry.source_info;
                let kind = OutputErrorKind::Io(e.to_string());
                OutputError::new(source_info, kind)
            })?;
        }
        Some(out) => out.write(&output, stdout, append).map_err(|e| {
            let filename = if let Output::File(filename) = out {
                filename.display().to_string()
            } else {
                "stdout".to_string()
            };
            let source_info = last_entry.source_info;
            let kind = OutputErrorKind::Io(format!("{filename} can not be written ({})", e));
            OutputError::new(source_info, kind)
        })?,
    }
    Ok(())
}

/// Returns `true` if `bytes` is a binary content, false otherwise.
///
/// For the implementation, we use a simple heuristic on the buffer: just check the presence of NULL
/// in the first 2000 bytes to determine if the content if binary or not.
///
/// See <https://github.com/curl/curl/pull/1512>
/// and <https://github.com/curl/curl/blob/721941aadf4adf4f6aeb3f4c0ab489bb89610c36/src/tool_cb_wrt.c#L209>
fn is_binary(bytes: &[u8]) -> bool {
    let len = min(2000, bytes.len());
    for c in &bytes[..len] {
        if *c == 0 {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use std::time::Duration;

    use hurl_core::ast::SourceInfo;
    use hurl_core::reader::Pos;

    use crate::http::{Call, Header, HeaderVec, HttpVersion, Request, Response, Url};
    use crate::output::write_last_body;
    use crate::runner::{EntryResult, HurlResult, Output};
    use crate::util::term::{Stdout, WriteMode};

    fn default_response() -> Response {
        Response {
            version: HttpVersion::Http10,
            status: 200,
            headers: HeaderVec::new(),
            body: vec![],
            duration: Default::default(),
            url: Url::from_str("http://localhost").unwrap(),
            certificate: None,
            ip_addr: Default::default(),
        }
    }

    fn hurl_result_json() -> HurlResult {
        let mut headers = HeaderVec::new();
        headers.push(Header::new("x-foo", "xxx"));
        headers.push(Header::new("x-bar", "yyy0"));
        headers.push(Header::new("x-bar", "yyy1"));
        headers.push(Header::new("x-bar", "yyy2"));
        headers.push(Header::new("x-baz", "zzz"));

        HurlResult {
            entries: vec![
                EntryResult {
                    entry_index: 1,
                    source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                    calls: vec![Call {
                        request: Request {
                            url: Url::from_str("https://foo.com").unwrap(),
                            method: "GET".to_string(),
                            headers: HeaderVec::new(),
                            body: vec![],
                        },
                        response: default_response(),
                        timings: Default::default(),
                    }],
                    ..Default::default()
                },
                EntryResult {
                    entry_index: 2,
                    source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                    calls: vec![Call {
                        request: Request {
                            url: Url::from_str("https://bar.com").unwrap(),
                            method: "GET".to_string(),
                            headers: HeaderVec::new(),
                            body: vec![],
                        },
                        response: default_response(),
                        timings: Default::default(),
                    }],
                    ..Default::default()
                },
                EntryResult {
                    entry_index: 3,
                    source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                    calls: vec![Call {
                        request: Request {
                            url: Url::from_str("https://baz.com").unwrap(),
                            method: "GET".to_string(),
                            headers: HeaderVec::new(),
                            body: vec![],
                        },
                        response: Response {
                            version: HttpVersion::Http3,
                            status: 204,
                            headers,
                            body: b"{\"say\": \"Hello World!\"}".into(),
                            duration: Default::default(),
                            url: Url::from_str("https://baz.com").unwrap(),
                            certificate: None,
                            ip_addr: Default::default(),
                        },
                        timings: Default::default(),
                    }],
                    ..Default::default()
                },
            ],
            duration: Duration::from_millis(100),
            success: true,
            ..Default::default()
        }
    }

    #[test]
    fn write_last_body_with_headers() {
        let result = hurl_result_json();
        let include_header = true;
        let color = false;
        let output = Some(Output::Stdout);
        let mut stdout = Stdout::new(WriteMode::Buffered);

        write_last_body(
            &result,
            include_header,
            color,
            output.as_ref(),
            &mut stdout,
            true,
        )
        .unwrap();
        let stdout = String::from_utf8(stdout.buffer().to_vec()).unwrap();
        assert_eq!(
            stdout,
            "HTTP/3 204\n\
             x-foo: xxx\n\
             x-bar: yyy0\n\
             x-bar: yyy1\n\
             x-bar: yyy2\n\
             x-baz: zzz\n\
             \n\
             {\"say\": \"Hello World!\"}"
        );
    }
}
