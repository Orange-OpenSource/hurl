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
use std::collections::HashMap;
use std::path::Path;

use hurl_core::typing::Count;

use crate::http::client::all_cookies;
use crate::http::{
    Body, ClientOptions, Cookie, FileParam, Header, HeaderVec, IpResolve, Method, MultipartParam,
    Param, RequestSpec, RequestedHttpVersion, CONTENT_TYPE,
};
use crate::runner::Output;
use crate::util::path::ContextDir;

/// Represents a curl command, with arguments.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CurlCmd {
    /// The args of this command.
    args: Vec<String>,
}

impl fmt::Display for CurlCmd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.args.join(" "))
    }
}

impl Default for CurlCmd {
    fn default() -> Self {
        CurlCmd {
            args: vec!["curl".to_string()],
        }
    }
}

impl CurlCmd {
    /// Creates a new curl command, based on an HTTP request, cookies, a context directory, output
    /// and runner options.
    pub fn new(
        request_spec: &RequestSpec,
        cookies: &[Cookie],
        context_dir: &ContextDir,
        output: Option<&Output>,
        options: &ClientOptions,
    ) -> Self {
        let mut args = vec!["curl".to_string()];

        let mut params = method_params(request_spec);
        args.append(&mut params);

        let options_headers = options
            .headers
            .iter()
            .map(|h| h.as_str())
            .collect::<Vec<&str>>();
        let headers = &request_spec.headers.aggregate_raw_headers(&options_headers);
        let mut params = headers_params(
            headers,
            request_spec.implicit_content_type.as_deref(),
            &request_spec.body,
        );
        args.append(&mut params);

        let mut params = body_params(request_spec, context_dir);
        args.append(&mut params);

        let mut params = cookies_params(request_spec, cookies);
        args.append(&mut params);

        let mut params = other_options_params(context_dir, output, options);
        args.append(&mut params);

        let mut params = url_param(request_spec);
        args.append(&mut params);

        CurlCmd { args }
    }
}

/// Returns the curl args corresponding to the HTTP method, from a request spec.
fn method_params(request_spec: &RequestSpec) -> Vec<String> {
    let has_body = !request_spec.multipart.is_empty()
        || !request_spec.form.is_empty()
        || !request_spec.body.bytes().is_empty();
    request_spec.method.curl_args(has_body)
}

/// Returns the curl args corresponding to the HTTP headers, from a list of headers,
/// an optional implicit content type, and the request body.
fn headers_params(
    headers: &HeaderVec,
    implicit_content_type: Option<&str>,
    body: &Body,
) -> Vec<String> {
    let mut args = vec![];

    for header in headers.iter() {
        args.append(&mut header.curl_args());
    }

    let has_explicit_content_type = headers.contains_key(CONTENT_TYPE);
    if has_explicit_content_type {
        return args;
    }

    if let Some(content_type) = implicit_content_type {
        if content_type != "application/x-www-form-urlencoded"
            && content_type != "multipart/form-data"
        {
            args.push("--header".to_string());
            args.push(format!("'{}: {content_type}'", CONTENT_TYPE));
        }
    } else if !body.bytes().is_empty() {
        match body {
            Body::Text(_) => {
                args.push("--header".to_string());
                args.push(format!("'{}:'", CONTENT_TYPE));
            }
            Body::Binary(_) => {
                args.push("--header".to_string());
                args.push(format!("'{}: application/octet-stream'", CONTENT_TYPE));
            }
            Body::File(_, _) => {
                args.push("--header".to_string());
                args.push(format!("'{}:'", CONTENT_TYPE));
            }
        }
    }
    args
}

/// Returns the curl args corresponding to the request body, from a request spec.
fn body_params(request_spec: &RequestSpec, context_dir: &ContextDir) -> Vec<String> {
    let mut args = vec![];

    for param in request_spec.form.iter() {
        args.push("--data".to_string());
        args.push(format!("'{}'", param.curl_arg_escape()));
    }
    for param in request_spec.multipart.iter() {
        args.push("--form".to_string());
        args.push(format!("'{}'", param.curl_arg(context_dir)));
    }

    if request_spec.body.bytes().is_empty() {
        return args;
    }

    // See <https://curl.se/docs/manpage.html#-d> and <https://curl.se/docs/manpage.html#--data-binary>:
    //
    // > -d, --data <data>
    // > ...
    // > If you start the data with the letter @, the rest should be a file name to read the
    // > data from, or - if you want curl to read the data from stdin. Posting data from a
    // > file named 'foobar' would thus be done with -d, --data @foobar. When -d, --data is
    // > told to read from a file like that, carriage returns and newlines will be stripped
    // > out. If you do not want the @ character to have a special interpretation use
    // > --data-raw instead.
    // > ...
    // > --data-binary <data>
    // >
    // > (HTTP) This posts data exactly as specified with no extra processing whatsoever.
    //
    // In summary: if the payload is a file (@foo.bin), we must use --data-binary option in
    // order to curl to not process the data sent.
    let param = match request_spec.body {
        Body::File(_, _) => "--data-binary",
        _ => "--data",
    };
    args.push(param.to_string());
    args.push(request_spec.body.curl_arg(context_dir));

    args
}

/// Returns the curl args corresponding to a list of cookies.
fn cookies_params(request_spec: &RequestSpec, cookies: &[Cookie]) -> Vec<String> {
    let mut args = vec![];

    let cookies = all_cookies(cookies, request_spec);
    if !cookies.is_empty() {
        args.push("--cookie".to_string());
        args.push(format!(
            "'{}'",
            cookies
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join("; ")
        ));
    }
    args
}

/// Returns the curl args corresponding to run options.
fn other_options_params(
    context_dir: &ContextDir,
    output: Option<&Output>,
    options: &ClientOptions,
) -> Vec<String> {
    let mut args = options.curl_args();

    // --output is not an option of the HTTP client, we deal with it here:
    match output {
        Some(Output::File(filename)) => {
            let filename = context_dir.resolved_path(filename);
            args.push("--output".to_string());
            args.push(filename.to_string_lossy().to_string());
        }
        Some(Output::Stdout) => {
            args.push("--output".to_string());
            args.push("-".to_string());
        }
        None => {}
    }
    args
}

/// Returns the curl args corresponding to the URL, from a request spec.
fn url_param(request_spec: &RequestSpec) -> Vec<String> {
    let mut args = vec![];

    let querystring = if request_spec.querystring.is_empty() {
        String::new()
    } else {
        let params = request_spec
            .querystring
            .iter()
            .map(|p| p.curl_arg_escape())
            .collect::<Vec<String>>();
        params.join("&")
    };
    let url = if querystring.as_str() == "" {
        request_spec.url.raw()
    } else if request_spec.url.raw().contains('?') {
        format!("{}&{}", request_spec.url.raw(), querystring)
    } else {
        format!("{}?{}", request_spec.url.raw(), querystring)
    };
    let url = format!("'{url}'");

    // curl support "globbing" <https://everything.curl.dev/cmdline/urls/globbing.html>
    // {,},[,] have special meaning to curl, in order to support templating.
    // We have two options:
    // - either we encode {,},[,] to %7b,%7d,%5b,%%5d
    // - or we let the url "as-it" and use curl [`--globoff`](https://curl.se/docs/manpage.html#-g) option.
    // We're going with the second one!
    if url.contains('{') || url.contains('}') || url.contains('[') || url.contains(']') {
        args.push("--globoff".to_string());
    }
    args.push(url);
    args
}

fn encode_byte(b: u8) -> String {
    format!("\\x{b:02x}")
}

/// Encode bytes to a shell string.
fn encode_bytes(bytes: &[u8]) -> String {
    bytes.iter().map(|b| encode_byte(*b)).collect()
}

impl Method {
    /// Returns the curl args for HTTP method, given the request has a body or not.
    fn curl_args(&self, has_body: bool) -> Vec<String> {
        match self.0.as_str() {
            "GET" => {
                if has_body {
                    vec!["--request".to_string(), "GET".to_string()]
                } else {
                    vec![]
                }
            }
            "HEAD" => vec!["--head".to_string()],
            "POST" => {
                if has_body {
                    vec![]
                } else {
                    vec!["--request".to_string(), "POST".to_string()]
                }
            }
            s => vec!["--request".to_string(), s.to_string()],
        }
    }
}

impl Header {
    fn curl_args(&self) -> Vec<String> {
        let name = &self.name;
        let value = &self.value;
        vec![
            "--header".to_string(),
            if self.value.is_empty() {
                encode_shell_string(&format!("{name};"))
            } else {
                encode_shell_string(&format!("{name}: {value}"))
            },
        ]
    }
}

impl Param {
    fn curl_arg_escape(&self) -> String {
        let name = &self.name;
        let value = escape_url(&self.value);
        format!("{name}={value}")
    }

    fn curl_arg(&self) -> String {
        let name = &self.name;
        let value = &self.value;
        format!("{name}={value}")
    }
}

impl MultipartParam {
    fn curl_arg(&self, context_dir: &ContextDir) -> String {
        match self {
            MultipartParam::Param(param) => param.curl_arg(),
            MultipartParam::FileParam(FileParam {
                name,
                filename,
                content_type,
                ..
            }) => {
                let path = context_dir.resolved_path(Path::new(filename));
                let value = format!("@{};type={}", path.to_string_lossy(), content_type);
                format!("{name}={value}")
            }
        }
    }
}

impl Body {
    fn curl_arg(&self, context_dir: &ContextDir) -> String {
        match self {
            Body::Text(s) => encode_shell_string(s),
            Body::Binary(bytes) => format!("$'{}'", encode_bytes(bytes)),
            Body::File(_, filename) => {
                let path = context_dir.resolved_path(Path::new(filename));
                format!("'@{}'", path.to_string_lossy())
            }
        }
    }
}

impl ClientOptions {
    /// Returns the list of options for the curl command line equivalent to this [`ClientOptions`].
    fn curl_args(&self) -> Vec<String> {
        let mut arguments = vec![];

        if let Some(ref aws_sigv4) = self.aws_sigv4 {
            arguments.push("--aws-sigv4".to_string());
            arguments.push(aws_sigv4.clone());
        }
        if let Some(ref cacert_file) = self.cacert_file {
            arguments.push("--cacert".to_string());
            arguments.push(cacert_file.clone());
        }
        if let Some(ref client_cert_file) = self.client_cert_file {
            arguments.push("--cert".to_string());
            arguments.push(client_cert_file.clone());
        }
        if let Some(ref client_key_file) = self.client_key_file {
            arguments.push("--key".to_string());
            arguments.push(client_key_file.clone());
        }
        if self.compressed {
            arguments.push("--compressed".to_string());
        }
        if self.connect_timeout != ClientOptions::default().connect_timeout {
            arguments.push("--connect-timeout".to_string());
            arguments.push(self.connect_timeout.as_secs().to_string());
        }
        for connect in self.connects_to.iter() {
            arguments.push("--connect-to".to_string());
            arguments.push(connect.clone());
        }
        if let Some(ref cookie_file) = self.cookie_input_file {
            arguments.push("--cookie".to_string());
            arguments.push(cookie_file.clone());
        }
        match self.http_version {
            RequestedHttpVersion::Default => {}
            RequestedHttpVersion::Http10 => arguments.push("--http1.0".to_string()),
            RequestedHttpVersion::Http11 => arguments.push("--http1.1".to_string()),
            RequestedHttpVersion::Http2 => arguments.push("--http2".to_string()),
            RequestedHttpVersion::Http3 => arguments.push("--http3".to_string()),
        }
        if self.insecure {
            arguments.push("--insecure".to_string());
        }
        match self.ip_resolve {
            IpResolve::Default => {}
            IpResolve::IpV4 => arguments.push("--ipv4".to_string()),
            IpResolve::IpV6 => arguments.push("--ipv6".to_string()),
        }
        if self.follow_location_trusted {
            arguments.push("--location-trusted".to_string());
        } else if self.follow_location {
            arguments.push("--location".to_string());
        }
        if let Some(max_filesize) = self.max_filesize {
            arguments.push("--max-filesize".to_string());
            arguments.push(max_filesize.to_string());
        }
        if let Some(max_speed) = self.max_recv_speed {
            arguments.push("--limit-rate".to_string());
            arguments.push(max_speed.to_string());
        }
        // We don't implement --limit-rate for self.max_send_speed as curl limit-rate seems
        // to limit both upload and download speed. There is no distinct option..
        if self.max_redirect != ClientOptions::default().max_redirect {
            let max_redirect = match self.max_redirect {
                Count::Finite(n) => n as i32,
                Count::Infinite => -1,
            };
            arguments.push("--max-redirs".to_string());
            arguments.push(max_redirect.to_string());
        }
        if let Some(filename) = &self.netrc_file {
            arguments.push("--netrc-file".to_string());
            arguments.push(format!("'{filename}'"));
        }
        if self.netrc_optional {
            arguments.push("--netrc-optional".to_string());
        }
        if self.netrc {
            arguments.push("--netrc".to_string());
        }
        if self.path_as_is {
            arguments.push("--path-as-is".to_string());
        }
        if let Some(ref proxy) = self.proxy {
            arguments.push("--proxy".to_string());
            arguments.push(format!("'{proxy}'"));
        }
        for resolve in self.resolves.iter() {
            arguments.push("--resolve".to_string());
            arguments.push(resolve.clone());
        }
        if self.ssl_no_revoke {
            arguments.push("--ssl-no-revoke".to_string());
        }
        if self.timeout != ClientOptions::default().timeout {
            arguments.push("--timeout".to_string());
            arguments.push(self.timeout.as_secs().to_string());
        }
        if let Some(ref unix_socket) = self.unix_socket {
            arguments.push("--unix-socket".to_string());
            arguments.push(format!("'{unix_socket}'"));
        }
        if let Some(ref user) = self.user {
            arguments.push("--user".to_string());
            arguments.push(format!("'{user}'"));
        }
        if let Some(ref user_agent) = self.user_agent {
            arguments.push("--user-agent".to_string());
            arguments.push(format!("'{user_agent}'"));
        }
        arguments
    }
}

fn escape_url(s: &str) -> String {
    percent_encoding::percent_encode(s.as_bytes(), percent_encoding::NON_ALPHANUMERIC).to_string()
}

fn encode_shell_string(s: &str) -> String {
    // $'...' form will be used to encode escaped sequence
    if escape_mode(s) {
        let escaped = escape_string(s);
        format!("$'{escaped}'")
    } else {
        format!("'{s}'")
    }
}

// the shell string must be in escaped mode ($'...')
// if it contains \n, \t or '
fn escape_mode(s: &str) -> bool {
    for c in s.chars() {
        if c == '\n' || c == '\t' || c == '\'' {
            return true;
        }
    }
    false
}

fn escape_string(s: &str) -> String {
    let mut escaped_sequences = HashMap::new();
    escaped_sequences.insert('\n', "\\n");
    escaped_sequences.insert('\t', "\\t");
    escaped_sequences.insert('\'', "\\'");
    escaped_sequences.insert('\\', "\\\\");

    let mut escaped = String::new();
    for c in s.chars() {
        match escaped_sequences.get(&c) {
            None => escaped.push(c),
            Some(escaped_seq) => escaped.push_str(escaped_seq),
        }
    }
    escaped
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::str::FromStr;
    use std::time::Duration;

    use hurl_core::typing::BytesPerSec;

    use super::*;
    use crate::http::{HeaderVec, Url};

    #[test]
    fn hello_request_with_default_options() {
        let mut request = RequestSpec {
            method: Method("GET".to_string()),
            url: Url::from_str("http://localhost:8000/hello").unwrap(),
            ..Default::default()
        };

        let context_dir = &ContextDir::default();
        let cookies = vec![];
        let options = ClientOptions::default();
        let output = None;

        let cmd = CurlCmd::new(&request, &cookies, context_dir, output.as_ref(), &options);
        assert_eq!(cmd.to_string(), "curl 'http://localhost:8000/hello'");

        // Same requests with some output:
        let output = Some(Output::new("foo.out"));
        let cmd = CurlCmd::new(&request, &cookies, context_dir, output.as_ref(), &options);
        assert_eq!(
            cmd.to_string(),
            "curl \
            --output foo.out \
            'http://localhost:8000/hello'"
        );

        // With some headers
        let mut headers = HeaderVec::new();
        headers.push(Header::new("User-Agent", "iPhone"));
        headers.push(Header::new("Foo", "Bar"));
        request.headers = headers;
        let cmd = CurlCmd::new(&request, &cookies, context_dir, output.as_ref(), &options);
        assert_eq!(
            cmd.to_string(),
            "curl \
            --header 'User-Agent: iPhone' \
            --header 'Foo: Bar' \
            --output foo.out \
            'http://localhost:8000/hello'"
        );

        // With some cookies:
        let cookies = vec![
            Cookie {
                domain: "localhost".to_string(),
                include_subdomain: "TRUE".to_string(),
                path: "/".to_string(),
                https: "FALSE".to_string(),
                expires: "0".to_string(),
                name: "cookie1".to_string(),
                value: "valueA".to_string(),
                http_only: false,
            },
            Cookie {
                domain: "localhost".to_string(),
                include_subdomain: "FALSE".to_string(),
                path: "/".to_string(),
                https: "FALSE".to_string(),
                expires: "1".to_string(),
                name: "cookie2".to_string(),
                value: String::new(),
                http_only: true,
            },
        ];
        let cmd = CurlCmd::new(&request, &cookies, context_dir, output.as_ref(), &options);
        assert_eq!(
            cmd.to_string(),
            "curl \
            --header 'User-Agent: iPhone' \
            --header 'Foo: Bar' \
            --cookie 'cookie1=valueA' \
            --output foo.out \
            'http://localhost:8000/hello'"
        );
    }

    #[test]
    fn hello_request_with_options() {
        let request = RequestSpec {
            method: Method("GET".to_string()),
            url: Url::from_str("http://localhost:8000/hello").unwrap(),
            ..Default::default()
        };

        let context_dir = &ContextDir::default();
        let cookies = vec![];
        let options = ClientOptions {
            allow_reuse: true,
            aws_sigv4: None,
            cacert_file: None,
            client_cert_file: None,
            client_key_file: None,
            compressed: true,
            connect_timeout: Duration::from_secs(20),
            connects_to: vec!["example.com:443:host-47.example.com:443".to_string()],
            cookie_input_file: Some("cookie_file".to_string()),
            follow_location: true,
            follow_location_trusted: false,
            headers: vec![
                "Test-Header-1: content-1".to_string(),
                "Test-Header-2: content-2".to_string(),
                "Test-Header-Empty:".to_string(),
            ],
            http_version: RequestedHttpVersion::Http10,
            insecure: true,
            ip_resolve: IpResolve::IpV6,
            max_filesize: None,
            max_recv_speed: Some(BytesPerSec(8000)),
            max_redirect: Count::Finite(10),
            max_send_speed: Some(BytesPerSec(8000)),
            netrc: false,
            netrc_file: Some("/var/run/netrc".to_string()),
            netrc_optional: true,
            path_as_is: true,
            proxy: Some("localhost:3128".to_string()),
            no_proxy: None,
            resolves: vec![
                "foo.com:80:192.168.0.1".to_string(),
                "bar.com:443:127.0.0.1".to_string(),
            ],
            ssl_no_revoke: false,
            timeout: Duration::from_secs(10),
            unix_socket: Some("/var/run/example.sock".to_string()),
            user: Some("user:password".to_string()),
            user_agent: Some("my-useragent".to_string()),
            verbosity: None,
        };

        let cmd = CurlCmd::new(&request, &cookies, context_dir, None, &options);
        assert_eq!(
            cmd.to_string(),
            "curl \
        --header 'Test-Header-1: content-1' \
        --header 'Test-Header-2: content-2' \
        --header 'Test-Header-Empty;' \
        --compressed \
        --connect-timeout 20 \
        --connect-to example.com:443:host-47.example.com:443 \
        --cookie cookie_file \
        --http1.0 \
        --insecure \
        --ipv6 \
        --location \
        --limit-rate 8000 \
        --max-redirs 10 \
        --netrc-file '/var/run/netrc' \
        --netrc-optional \
        --path-as-is \
        --proxy 'localhost:3128' \
        --resolve foo.com:80:192.168.0.1 \
        --resolve bar.com:443:127.0.0.1 \
        --timeout 10 \
        --unix-socket '/var/run/example.sock' \
        --user 'user:password' \
        --user-agent 'my-useragent' \
        'http://localhost:8000/hello'"
        );
    }

    #[test]
    fn url_with_dot() {
        let request = RequestSpec {
            method: Method("GET".to_string()),
            url: Url::from_str("https://example.org/hello/../to/../your/../file").unwrap(),
            ..Default::default()
        };

        let context_dir = &ContextDir::default();
        let cookies = vec![];
        let options = ClientOptions::default();
        let output = None;

        let cmd = CurlCmd::new(&request, &cookies, context_dir, output.as_ref(), &options);
        assert_eq!(
            cmd.to_string(),
            "curl 'https://example.org/hello/../to/../your/../file'"
        );
    }

    #[test]
    fn url_with_curl_glob() {
        let request = RequestSpec {
            method: Method("GET".to_string()),
            url: Url::from_str("http://foo.com?param1=value1&param2={bar}").unwrap(),
            ..Default::default()
        };

        let context_dir = &ContextDir::default();
        let cookies = vec![];
        let options = ClientOptions::default();
        let output = None;

        let cmd = CurlCmd::new(&request, &cookies, context_dir, output.as_ref(), &options);
        assert_eq!(
            cmd.to_string(),
            "curl \
             --globoff \
            'http://foo.com?param1=value1&param2={bar}'"
        );
    }

    #[test]
    fn query_request() {
        let mut request = RequestSpec {
            method: Method("GET".to_string()),
            url: Url::from_str("http://localhost:8000/querystring-params").unwrap(),
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
        };

        let context_dir = &ContextDir::default();
        let cookies = vec![];
        let options = ClientOptions::default();
        let output = None;

        let cmd = CurlCmd::new(&request, &cookies, context_dir, output.as_ref(), &options);
        assert_eq!(
            cmd.to_string(),
            "curl 'http://localhost:8000/querystring-params?param1=value1&param2=a%20b'",
        );

        // Add som query param in the URL
        request.url =
            Url::from_str("http://localhost:8000/querystring-params?param3=foo&param4=bar")
                .unwrap();
        let cmd = CurlCmd::new(&request, &cookies, context_dir, output.as_ref(), &options);
        assert_eq!(
            cmd.to_string(),
            "curl 'http://localhost:8000/querystring-params?param3=foo&param4=bar&param1=value1&param2=a%20b'",
        );
    }

    #[test]
    fn form_request() {
        let mut headers = HeaderVec::new();
        headers.push(Header::new(
            "Content-Type",
            "application/x-www-form-urlencoded",
        ));

        let request = RequestSpec {
            method: Method("POST".to_string()),
            url: Url::from_str("http://localhost/form-params").unwrap(),
            headers,
            form: vec![Param::new("param1", "value1"), Param::new("param2", "a b")],
            implicit_content_type: Some("multipart/form-data".to_string()),
            ..Default::default()
        };

        let context_dir = &ContextDir::default();
        let cookies = vec![];
        let options = ClientOptions::default();
        let output = None;

        let cmd = CurlCmd::new(&request, &cookies, context_dir, output.as_ref(), &options);
        assert_eq!(
            cmd.to_string(),
            "curl \
            --header 'Content-Type: application/x-www-form-urlencoded' \
            --data 'param1=value1' \
            --data 'param2=a%20b' \
            'http://localhost/form-params'"
        );
    }

    #[test]
    fn json_request() {
        let mut headers = HeaderVec::new();
        headers.push(Header::new("content-type", "application/vnd.api+json"));
        let mut request = RequestSpec {
            method: Method("POST".to_string()),
            url: Url::from_str("http://localhost/json").unwrap(),
            headers,
            body: Body::Text(String::new()),
            implicit_content_type: Some("application/json".to_string()),
            ..Default::default()
        };

        let context_dir = &ContextDir::default();
        let cookies = vec![];
        let options = ClientOptions::default();
        let output = None;

        let cmd = CurlCmd::new(&request, &cookies, context_dir, output.as_ref(), &options);
        assert_eq!(
            cmd.to_string(),
            "curl \
            --request POST \
            --header 'content-type: application/vnd.api+json' \
            'http://localhost/json'"
        );

        // Add a non-empty body
        request.body = Body::Text("{\"foo\":\"bar\"}".to_string());
        let cmd = CurlCmd::new(&request, &cookies, context_dir, output.as_ref(), &options);
        assert_eq!(
            cmd.to_string(),
            "curl \
            --header 'content-type: application/vnd.api+json' \
            --data '{\"foo\":\"bar\"}' \
            'http://localhost/json'"
        );

        // Change method
        request.method = Method("PUT".to_string());
        let cmd = CurlCmd::new(&request, &cookies, context_dir, output.as_ref(), &options);
        assert_eq!(
            cmd.to_string(),
            "curl \
            --request PUT \
            --header 'content-type: application/vnd.api+json' \
            --data '{\"foo\":\"bar\"}' \
            'http://localhost/json'"
        );
    }

    #[test]
    fn post_binary_file() {
        let request = RequestSpec {
            method: Method("POST".to_string()),
            url: Url::from_str("http://localhost:8000/hello").unwrap(),
            body: Body::File(b"Hello World!".to_vec(), "foo.bin".to_string()),
            ..Default::default()
        };

        let context_dir = &ContextDir::default();
        let cookies = vec![];
        let options = ClientOptions::default();
        let output = None;

        let cmd = CurlCmd::new(&request, &cookies, context_dir, output.as_ref(), &options);
        assert_eq!(
            cmd.to_string(),
            "curl \
            --header 'Content-Type:' \
            --data-binary '@foo.bin' \
            'http://localhost:8000/hello'"
        );
    }

    #[test]
    fn test_encode_byte() {
        assert_eq!(encode_byte(1), "\\x01".to_string());
        assert_eq!(encode_byte(32), "\\x20".to_string());
    }

    #[test]
    fn header_curl_args() {
        assert_eq!(
            Header::new("Host", "example.com").curl_args(),
            vec!["--header".to_string(), "'Host: example.com'".to_string()]
        );
        assert_eq!(
            Header::new("If-Match", "\"e0023aa4e\"").curl_args(),
            vec![
                "--header".to_string(),
                "'If-Match: \"e0023aa4e\"'".to_string()
            ]
        );
    }

    #[test]
    fn param_curl_args() {
        assert_eq!(
            Param {
                name: "param1".to_string(),
                value: "value1".to_string(),
            }
            .curl_arg(),
            "param1=value1".to_string()
        );
        assert_eq!(
            Param {
                name: "param2".to_string(),
                value: String::new(),
            }
            .curl_arg(),
            "param2=".to_string()
        );
        assert_eq!(
            Param {
                name: "param3".to_string(),
                value: "a=b".to_string(),
            }
            .curl_arg_escape(),
            "param3=a%3Db".to_string()
        );
        assert_eq!(
            Param {
                name: "param4".to_string(),
                value: "1,2,3".to_string(),
            }
            .curl_arg_escape(),
            "param4=1%2C2%2C3".to_string()
        );
    }

    #[test]
    fn test_encode_body() {
        let current_dir = Path::new("/tmp");
        let file_root = Path::new("/tmp");
        let context_dir = ContextDir::new(current_dir, file_root);
        assert_eq!(
            Body::Text("hello".to_string()).curl_arg(&context_dir),
            "'hello'".to_string()
        );

        if cfg!(unix) {
            assert_eq!(
                Body::File(vec![], "filename".to_string()).curl_arg(&context_dir),
                "'@/tmp/filename'".to_string()
            );
        }

        assert_eq!(
            Body::Binary(vec![1, 2, 3]).curl_arg(&context_dir),
            "$'\\x01\\x02\\x03'".to_string()
        );
    }

    #[test]
    fn test_encode_shell_string() {
        assert_eq!(encode_shell_string("hello"), "'hello'");
        assert_eq!(encode_shell_string("\\n"), "'\\n'");
        assert_eq!(encode_shell_string("'"), "$'\\''");
        assert_eq!(encode_shell_string("\\'"), "$'\\\\\\''");
        assert_eq!(encode_shell_string("\n"), "$'\\n'");
    }

    #[test]
    fn test_escape_string() {
        assert_eq!(escape_string("hello"), "hello");
        assert_eq!(escape_string("\\n"), "\\\\n");
        assert_eq!(escape_string("'"), "\\'");
        assert_eq!(escape_string("\\'"), "\\\\\\'");
        assert_eq!(escape_string("\n"), "\\n");
    }

    #[test]
    fn test_escape_mode() {
        assert!(!escape_mode("hello"));
        assert!(!escape_mode("\\"));
        assert!(escape_mode("'"));
        assert!(escape_mode("\n"));
    }
}
