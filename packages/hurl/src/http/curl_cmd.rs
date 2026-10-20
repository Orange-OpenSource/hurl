/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2026 Orange
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

use crate::http::TlsVersion;
use crate::runner::Output;
use crate::util::path::ContextDir;
use crate::util::redacted::Redact;
use hurl_core::types::Count;

use super::cookie_store::CookieStore;
use super::header::{CONTENT_TYPE, Header, HeaderVec};
use super::options::ClientOptions;
use super::param::Param;
use super::request::{CredentialForwarding, FollowLocation, IpResolve, RequestedHttpVersion};
use super::request_spec::{Body, FileParam, Method, MultipartParam, RequestSpec};

/// Represents a curl command, with arguments.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CurlCmd {
    /// The args of this command.
    args: Vec<Arg>,
    /// An optional command feeding the request body to curl's stdin (used when the body contains
    /// a NUL byte, which can not be embedded in a shell string).
    stdin: Option<Vec<u8>>,
}

/// A curl command argument.
///
/// Arguments are stored unencoded: shell encoding is only done when the command is rendered, so
/// any value (URL, headers, file paths etc...) is always safely escaped.
#[derive(Clone, Debug, PartialEq, Eq)]
enum Arg {
    /// An argument rendered as-is if it only contains shell safe characters, quoted otherwise
    /// (option names, file paths, `host:port` values etc...).
    Plain(String),
    /// An argument always rendered quoted (URL, headers, data etc...).
    Quoted(String),
    /// Raw bytes, rendered as `$'\x..'` (binary body).
    Bytes(Vec<u8>),
}

impl Arg {
    /// Constructs a plain curl arg (maybe quoted or not).
    fn plain(s: &str) -> Self {
        Arg::Plain(s.to_string())
    }

    /// Constructs a quoted curl arg (always quoted).
    fn quoted(s: &str) -> Self {
        Arg::Quoted(s.to_string())
    }

    /// Returns the shell encoded string of this argument, with `secrets` redacted.
    fn encode(&self, secrets: &[impl AsRef<str>]) -> String {
        match self {
            Arg::Plain(s) => {
                let s = s.redact(secrets);
                if is_shell_safe(&s) { s } else { quote(&s) }
            }
            Arg::Quoted(s) => quote(&s.redact(secrets)),
            Arg::Bytes(bytes) => format!("$'{}'", encode_bytes(bytes)),
        }
    }
}

impl Default for CurlCmd {
    fn default() -> Self {
        CurlCmd {
            args: vec![Arg::plain("curl")],
            stdin: None,
        }
    }
}

impl CurlCmd {
    /// Creates a new curl command, based on an HTTP request, cookies, a context directory, output
    /// and runner options.
    pub fn new(
        request_spec: &RequestSpec,
        cookie_store: &CookieStore,
        context_dir: &ContextDir,
        output: Option<&Output>,
        options: &ClientOptions,
    ) -> Self {
        let mut args = vec![Arg::plain("curl")];

        let mut params = method_params(request_spec, options.follow_location);
        args.append(&mut params);

        let mut headers = request_spec.headers.clone();
        headers.extend(&options.headers);
        // We need to filter added headers that have been explicitly removed by user
        headers.retain(|h| !options.no_headers.iter().any(|name| h.name_eq(name)));
        let mut params = headers_params(
            &headers,
            request_spec.implicit_content_type.as_deref(),
            &request_spec.body,
        );
        args.append(&mut params);

        let mut params = no_headers_params(&options.no_headers);
        args.append(&mut params);

        let (mut params, stdin) = body_params(request_spec, context_dir);
        args.append(&mut params);

        let mut params = cookies_params(request_spec, cookie_store);
        args.append(&mut params);

        let mut params = other_options_params(context_dir, output, options);
        args.append(&mut params);

        let mut params = url_param(request_spec);
        args.append(&mut params);

        CurlCmd { args, stdin }
    }

    /// Returns the shell command line of this curl command, with `secrets` redacted.
    ///
    /// Secrets are redacted on the unencoded arguments, before shell encoding, so a secret is
    /// redacted even if its encoded value is different (for instance, if it contains a quote).
    pub fn to_redacted_string(&self, secrets: &[impl AsRef<str>]) -> String {
        let args = self
            .args
            .iter()
            .map(|arg| arg.encode(secrets))
            .collect::<Vec<_>>()
            .join(" ");
        match &self.stdin {
            Some(bytes) => format!("printf '{}' | {args}", encode_bytes(bytes)),
            None => args,
        }
    }
}

/// Returns the curl args corresponding to the HTTP method, from a request spec.
fn method_params(request_spec: &RequestSpec, follow_location: FollowLocation) -> Vec<Arg> {
    let has_body = !request_spec.multipart.is_empty()
        || !request_spec.form.is_empty()
        || !request_spec.body.bytes().is_empty();
    request_spec.method.curl_args(has_body, follow_location)
}

/// Returns the curl args corresponding to the HTTP headers, from a list of headers,
/// an optional implicit content type, and the request body.
fn headers_params(
    headers: &HeaderVec,
    implicit_content_type: Option<&str>,
    body: &Body,
) -> Vec<Arg> {
    let mut args = vec![];

    for header in headers.iter() {
        args.append(&mut header.curl_args("--header"));
    }

    let has_explicit_content_type = headers.contains_key(CONTENT_TYPE);
    if has_explicit_content_type {
        return args;
    }

    if let Some(content_type) = implicit_content_type {
        if content_type != "application/x-www-form-urlencoded"
            && content_type != "multipart/form-data"
        {
            args.push(Arg::plain("--header"));
            args.push(Arg::quoted(&format!("{CONTENT_TYPE}: {content_type}")));
        }
    } else if !body.bytes().is_empty() {
        let content_type = match body {
            Body::Text(_) | Body::File(_, _) => "",
            Body::Binary(_) => " application/octet-stream",
        };
        args.push(Arg::plain("--header"));
        args.push(Arg::quoted(&format!("{CONTENT_TYPE}:{content_type}")));
    }
    args
}

/// Returns the curl args corresponding to removing some request headers.
fn no_headers_params(no_headers: &[String]) -> Vec<Arg> {
    let mut args = vec![];
    no_headers.iter().for_each(|name| {
        args.push(Arg::plain("--header"));
        args.push(Arg::quoted(&format!("{name}:")));
    });
    args
}

/// Returns the curl args corresponding to the request body, from a request spec.
fn body_params(
    request_spec: &RequestSpec,
    context_dir: &ContextDir,
) -> (Vec<Arg>, Option<Vec<u8>>) {
    let mut args = vec![];

    for param in request_spec.form.iter() {
        args.push(Arg::plain("--data"));
        args.push(Arg::quoted(&param.curl_arg_escape()));
    }
    for param in request_spec.multipart.iter() {
        args.push(Arg::plain("--form"));
        args.push(Arg::quoted(&param.curl_arg(context_dir)));
    }

    if request_spec.body.bytes().is_empty() {
        return (args, None);
    }

    // A body containing a NUL byte can not be passed as a shell string
    // argument ($'...' can not hold NUL): in this case, and only in this
    // case, we pipe the body to curl's stdin instead
    // (https://github.com/Orange-OpenSource/hurl/issues/5141).
    if !matches!(request_spec.body, Body::File(_, _)) && request_spec.body.bytes().contains(&0) {
        args.push(Arg::plain("--data-binary"));
        args.push(Arg::plain("@-"));
        return (args, Some(request_spec.body.bytes().to_vec()));
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
    args.push(Arg::plain(param));
    args.push(request_spec.body.curl_arg(context_dir));

    (args, None)
}

/// Returns the curl args corresponding to a list of cookies.
fn cookies_params(request_spec: &RequestSpec, cookie_store: &CookieStore) -> Vec<Arg> {
    // Constructs cookies values from current request
    let mut cookies_from_req = request_spec
        .cookies
        .iter()
        .map(|c| (c.name.as_str(), c.value.as_str()))
        .collect::<Vec<_>>();

    // Constructs cookies values from cookie store for this URL, that are not expired
    let mut cookies_from_store = cookie_store
        .cookies()
        .filter(|c| !c.is_expired())
        .filter(|c| c.match_domain(&request_spec.url))
        .map(|c| (c.name(), c.value()))
        .collect::<Vec<_>>();

    let mut all_cookies = vec![];
    all_cookies.append(&mut cookies_from_req);
    all_cookies.append(&mut cookies_from_store);
    if all_cookies.is_empty() {
        return vec![];
    }

    let value = all_cookies
        .iter()
        .map(|(name, value)| format!("{name}={value}"))
        .collect::<Vec<String>>()
        .join("; ");
    vec![Arg::plain("--cookie"), Arg::quoted(&value)]
}

/// Returns the curl args corresponding to run options.
fn other_options_params(
    context_dir: &ContextDir,
    output: Option<&Output>,
    options: &ClientOptions,
) -> Vec<Arg> {
    let mut args = options.curl_args();

    // --output is not an option of the HTTP client, we deal with it here:
    match output {
        Some(Output::File(filename)) => {
            let filename = context_dir.resolved_path(filename);
            args.push(Arg::plain("--output"));
            args.push(Arg::plain(&filename.to_string_lossy()));
        }
        Some(Output::Stdout) => {
            args.push(Arg::plain("--output"));
            args.push(Arg::plain("-"));
        }
        None => {}
    }
    args
}

/// Returns the curl args corresponding to the URL, from a request spec.
fn url_param(request_spec: &RequestSpec) -> Vec<Arg> {
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

    // curl support "globbing" <https://everything.curl.dev/cmdline/urls/globbing.html>
    // {,},[,] have special meaning to curl, in order to support templating.
    // We have two options:
    // - either we encode {,},[,] to %7b,%7d,%5b,%%5d
    // - or we let the url "as-it" and use curl [`--globoff`](https://curl.se/docs/manpage.html#-g) option.
    // We're going with the second one!
    if url.contains(['{', '}', '[', ']']) {
        args.push(Arg::plain("--globoff"));
    }
    args.push(Arg::quoted(&url));
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
    fn curl_args(&self, has_body: bool, follow_location: FollowLocation) -> Vec<Arg> {
        match self.0.as_str() {
            "GET" => {
                if has_body {
                    vec![Arg::plain("--request"), Arg::plain("GET")]
                } else {
                    vec![]
                }
            }
            "HEAD" => vec![Arg::plain("--head")],
            "POST" => {
                // In <https://curl.se/docs/manpage.html#-X>, `--location` and `--request/-X` does not well play together:
                //
                // > The method string you set with -X, --request is used for all requests, which if you for example use -L,
                // > --location may cause unintended side-effects when curl does not change request method according to the
                // > HTTP 30x response codes - and similar.
                //
                // When we use `--request POST` with curl, we're telling curl to make POST requests for every request. This
                // can interfere with `--location` option that can make GET requests following the initial POST. So, in the
                // case of a POST request with `--location` option, we don't force the method to let curl decides the right
                // method of the redirection steps.
                if has_body {
                    vec![]
                } else if matches!(follow_location, FollowLocation::Follow(_)) {
                    vec![Arg::plain("--data"), Arg::quoted("")]
                } else {
                    vec![Arg::plain("--request"), Arg::plain("POST")]
                }
            }
            s => vec![Arg::plain("--request"), Arg::plain(s)],
        }
    }
}

impl Header {
    fn curl_args(&self, option: &'static str) -> Vec<Arg> {
        let name = &self.name;
        let value = &self.value;
        let header = if value.is_empty() {
            format!("{name};")
        } else {
            format!("{name}: {value}")
        };
        vec![Arg::plain(option), Arg::quoted(&header)]
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
    fn curl_arg(&self, context_dir: &ContextDir) -> Arg {
        match self {
            Body::Text(s) => Arg::quoted(s.as_str()),
            Body::Binary(bytes) => Arg::Bytes(bytes.clone()),
            Body::File(_, filename) => {
                let path = context_dir.resolved_path(Path::new(filename));
                Arg::quoted(&format!("@{}", path.to_string_lossy()))
            }
        }
    }
}

impl ClientOptions {
    /// Returns the list of options for the curl command line equivalent to this [`ClientOptions`].
    fn curl_args(&self) -> Vec<Arg> {
        let mut args = vec![];

        if let Some(ref aws_sigv4) = self.aws_sigv4 {
            args.push(Arg::plain("--aws-sigv4"));
            args.push(Arg::plain(aws_sigv4));
        }
        if let Some(ref cacert_file) = self.cacert_file {
            args.push(Arg::plain("--cacert"));
            args.push(Arg::plain(cacert_file));
        }
        if let Some(ref client_cert_file) = self.client_cert_file {
            args.push(Arg::plain("--cert"));
            args.push(Arg::plain(client_cert_file));
        }
        if let Some(ref client_key_file) = self.client_key_file {
            args.push(Arg::plain("--key"));
            args.push(Arg::plain(client_key_file));
        }
        if self.compressed {
            args.push(Arg::plain("--compressed"));
        }
        if self.connect_timeout != ClientOptions::default().connect_timeout {
            args.push(Arg::plain("--connect-timeout"));
            args.push(Arg::plain(&self.connect_timeout.as_secs().to_string()));
        }
        for connect in self.connects_to.iter() {
            args.push(Arg::plain("--connect-to"));
            args.push(Arg::plain(connect));
        }
        if let Some(ref cookie_file) = self.cookie_input_file {
            args.push(Arg::plain("--cookie"));
            args.push(Arg::plain(cookie_file));
        }
        if self.digest {
            args.push(Arg::plain("--digest"));
        }
        match self.http_version {
            RequestedHttpVersion::Default => {}
            RequestedHttpVersion::Http10 => args.push(Arg::plain("--http1.0")),
            RequestedHttpVersion::Http11 => args.push(Arg::plain("--http1.1")),
            RequestedHttpVersion::Http2 => args.push(Arg::plain("--http2")),
            RequestedHttpVersion::Http2PriorKnowledge => {
                args.push(Arg::plain("--http2-prior-knowledge"));
            }
            RequestedHttpVersion::Http3 => args.push(Arg::plain("--http3")),
        }
        if self.insecure {
            args.push(Arg::plain("--insecure"));
        }
        match self.ip_resolve {
            IpResolve::Default => {}
            IpResolve::IpV4 => args.push(Arg::plain("--ipv4")),
            IpResolve::IpV6 => args.push(Arg::plain("--ipv6")),
        }
        match self.follow_location {
            FollowLocation::Follow(CredentialForwarding::OnlyInitialHost) => {
                args.push(Arg::plain("--location"));
            }
            FollowLocation::Follow(CredentialForwarding::AllHosts) => {
                args.push(Arg::plain("--location-trusted"));
            }
            FollowLocation::No => {}
        }
        if let Some(max_filesize) = self.max_filesize {
            args.push(Arg::plain("--max-filesize"));
            args.push(Arg::plain(&max_filesize.to_string()));
        }
        if let Some(max_speed) = self.max_recv_speed {
            args.push(Arg::plain("--limit-rate"));
            args.push(Arg::plain(&max_speed.to_string()));
        }
        // We don't implement --limit-rate for self.max_send_speed as curl limit-rate seems
        // to limit both upload and download speed. There is no distinct option..
        if self.max_redirect != ClientOptions::default().max_redirect {
            let max_redirect = match self.max_redirect {
                Count::Finite(n) => n as i32,
                Count::Infinite => -1,
            };
            args.push(Arg::plain("--max-redirs"));
            args.push(Arg::plain(&max_redirect.to_string()));
        }
        if self.timeout != ClientOptions::default().timeout {
            args.push(Arg::plain("--max-time"));
            args.push(Arg::plain(&self.timeout.as_secs().to_string()));
        }
        match self.tls_min {
            TlsVersion::Default => {}
            version => {
                // curl has no --tls-min option, minimum TLS version is
                // set with --tlsv1.X options.
                args.push(Arg::plain(&format!("--tlsv{version}")));
            }
        }
        match self.tls_max {
            TlsVersion::Default => {}
            version => {
                args.push(Arg::plain("--tls-max"));
                args.push(Arg::plain(&version.to_string()));
            }
        }
        if self.negotiate {
            args.push(Arg::plain("--negotiate"));
        }
        if let Some(filename) = &self.netrc_file {
            args.push(Arg::plain("--netrc-file"));
            args.push(Arg::quoted(filename));
        }
        if self.netrc_optional {
            args.push(Arg::plain("--netrc-optional"));
        }
        if self.netrc {
            args.push(Arg::plain("--netrc"));
        }
        if self.ntlm {
            args.push(Arg::plain("--ntlm"));
        }
        if self.path_as_is {
            args.push(Arg::plain("--path-as-is"));
        }
        if let Some(ref pinned_pub_key) = self.pinned_pub_key {
            args.push(Arg::plain("--pinnedpubkey"));
            args.push(Arg::plain(pinned_pub_key));
        }
        if let Some(ref proxy) = self.proxy {
            args.push(Arg::plain("--proxy"));
            args.push(Arg::quoted(proxy));
        }
        for header in self.proxy_headers.iter() {
            args.append(&mut header.curl_args("--proxy-header"));
        }
        for resolve in self.resolves.iter() {
            args.push(Arg::plain("--resolve"));
            args.push(Arg::plain(resolve));
        }
        if self.ssl_no_revoke {
            args.push(Arg::plain("--ssl-no-revoke"));
        }
        if let Some(ref unix_socket) = self.unix_socket {
            args.push(Arg::plain("--unix-socket"));
            args.push(Arg::quoted(unix_socket));
        }
        if let Some(ref user) = self.user {
            args.push(Arg::plain("--user"));
            args.push(Arg::quoted(user));
        }
        if let Some(ref user_agent) = self.user_agent {
            args.push(Arg::plain("--user-agent"));
            args.push(Arg::quoted(user_agent));
        }
        args
    }
}

fn escape_url(s: &str) -> String {
    percent_encoding::percent_encode(s.as_bytes(), percent_encoding::NON_ALPHANUMERIC).to_string()
}

/// Returns `true` if `s` can be used unquoted as a shell argument.
fn is_shell_safe(s: &str) -> bool {
    !s.is_empty()
        && s.chars()
            .all(|c| c.is_ascii_alphanumeric() || "-_./:=@,+%".contains(c))
}

/// Quotes `s` as a single shell argument.
///
/// A plain '...' string is used, except if `s` contains `'`, `\n` or `\t`: in this case, the
/// $'...' form is used, with escaped sequences.
fn quote(s: &str) -> String {
    if !s.contains(['\'', '\n', '\t']) {
        return format!("'{s}'");
    }
    let mut escaped = String::new();
    for c in s.chars() {
        match c {
            '\n' => escaped.push_str("\\n"),
            '\t' => escaped.push_str("\\t"),
            '\'' => escaped.push_str("\\'"),
            '\\' => escaped.push_str("\\\\"),
            c => escaped.push(c),
        }
    }
    format!("$'{escaped}'")
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::str::FromStr;
    use std::time::Duration;

    use hurl_core::types::BytesPerSec;

    use super::*;
    use crate::http::{HeaderVec, TlsVersion, Url};

    #[test]
    fn hello_request_with_default_options() {
        let secrets: &[&str] = &[];
        let mut request = RequestSpec {
            method: Method("GET".to_string()),
            url: Url::from_str("http://localhost:8000/hello").unwrap(),
            ..Default::default()
        };

        let context_dir = ContextDir::default();
        let cookie_store = CookieStore::new();
        let options = ClientOptions::default();
        let output = None;

        let cmd = CurlCmd::new(
            &request,
            &cookie_store,
            &context_dir,
            output.as_ref(),
            &options,
        );
        assert_eq!(
            cmd.to_redacted_string(secrets),
            "curl 'http://localhost:8000/hello'"
        );

        // Same requests with some output:
        let output = Some(Output::new("foo.out"));
        let cmd = CurlCmd::new(
            &request,
            &cookie_store,
            &context_dir,
            output.as_ref(),
            &options,
        );
        assert_eq!(
            cmd.to_redacted_string(secrets),
            "curl \
            --output foo.out \
            'http://localhost:8000/hello'"
        );

        // With some headers
        let mut headers = HeaderVec::new();
        headers.push(Header::new("User-Agent", "iPhone"));
        headers.push(Header::new("Foo", "Bar"));
        request.headers = headers;
        let cmd = CurlCmd::new(
            &request,
            &cookie_store,
            &context_dir,
            output.as_ref(),
            &options,
        );
        assert_eq!(
            cmd.to_redacted_string(secrets),
            "curl \
            --header 'User-Agent: iPhone' \
            --header 'Foo: Bar' \
            --output foo.out \
            'http://localhost:8000/hello'"
        );

        // With some cookies:
        let mut cookie_store = CookieStore::new();
        cookie_store
            .add_cookie("localhost  TRUE    /   FALSE   0   cookie1 valueA")
            .unwrap();
        cookie_store
            .add_cookie("localhost  FALSE    /   FALSE   1   cookie2 valueB")
            .unwrap();
        let cmd = CurlCmd::new(
            &request,
            &cookie_store,
            &context_dir,
            output.as_ref(),
            &options,
        );
        assert_eq!(
            cmd.to_redacted_string(secrets),
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
        let secrets: &[&str] = &[];
        let request = RequestSpec {
            method: Method("GET".to_string()),
            url: Url::from_str("http://localhost:8000/hello").unwrap(),
            ..Default::default()
        };

        let context_dir = ContextDir::default();
        let cookie_store = CookieStore::new();
        let mut headers = HeaderVec::new();
        headers.push(Header::new("Test-Header-1", "content-1"));
        headers.push(Header::new("Test-Header-2", "content-2"));
        headers.push(Header::new("Test-Header-Empty", ""));

        let mut proxy_headers = HeaderVec::new();
        proxy_headers.push(Header::new("X-Foo", "foo"));

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
            digest: false,
            follow_location: FollowLocation::Follow(CredentialForwarding::OnlyInitialHost),
            headers,
            http_version: RequestedHttpVersion::Http10,
            insecure: true,
            ip_resolve: IpResolve::IpV6,
            max_filesize: None,
            max_recv_speed: Some(BytesPerSec(8000)),
            max_redirect: Count::Finite(10),
            max_send_speed: Some(BytesPerSec(8000)),
            negotiate: true,
            netrc: false,
            netrc_file: Some("/var/run/netrc".to_string()),
            netrc_optional: true,
            no_headers: Vec::new(),
            ntlm: true,
            path_as_is: true,
            pinned_pub_key: None,
            proxy: Some("localhost:3128".to_string()),
            proxy_headers,
            no_proxy: None,
            resolves: vec![
                "foo.com:80:192.168.0.1".to_string(),
                "bar.com:443:127.0.0.1".to_string(),
            ],
            ssl_no_revoke: false,
            timeout: Duration::from_secs(10),
            tls_max: TlsVersion::Tls12,
            tls_min: TlsVersion::Tls10,
            unix_socket: Some("/var/run/example.sock".to_string()),
            use_cookie_store: true,
            user: Some("user:password".to_string()),
            user_agent: Some("my-useragent".to_string()),
            verbosity: None,
        };

        let cmd = CurlCmd::new(&request, &cookie_store, &context_dir, None, &options);
        assert_eq!(
            cmd.to_redacted_string(secrets),
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
        --max-time 10 \
        --tlsv1.0 \
        --tls-max 1.2 \
        --negotiate \
        --netrc-file '/var/run/netrc' \
        --netrc-optional \
        --ntlm \
        --path-as-is \
        --proxy 'localhost:3128' \
        --proxy-header 'X-Foo: foo' \
        --resolve foo.com:80:192.168.0.1 \
        --resolve bar.com:443:127.0.0.1 \
        --unix-socket '/var/run/example.sock' \
        --user 'user:password' \
        --user-agent 'my-useragent' \
        'http://localhost:8000/hello'"
        );
    }

    #[test]
    fn url_with_dot() {
        let secrets: &[&str] = &[];
        let request = RequestSpec {
            method: Method("GET".to_string()),
            url: Url::from_str("https://example.org/hello/../to/../your/../file").unwrap(),
            ..Default::default()
        };

        let context_dir = ContextDir::default();
        let cookies = CookieStore::new();
        let options = ClientOptions::default();
        let output = None;

        let cmd = CurlCmd::new(&request, &cookies, &context_dir, output.as_ref(), &options);
        assert_eq!(
            cmd.to_redacted_string(secrets),
            "curl 'https://example.org/hello/../to/../your/../file'"
        );
    }

    #[test]
    fn url_with_curl_glob() {
        let secrets: &[&str] = &[];
        let request = RequestSpec {
            method: Method("GET".to_string()),
            url: Url::from_str("http://foo.com?param1=value1&param2={bar}").unwrap(),
            ..Default::default()
        };

        let context_dir = ContextDir::default();
        let cookie_store = CookieStore::new();
        let options = ClientOptions::default();
        let output = None;

        let cmd = CurlCmd::new(
            &request,
            &cookie_store,
            &context_dir,
            output.as_ref(),
            &options,
        );
        assert_eq!(
            cmd.to_redacted_string(secrets),
            "curl \
             --globoff \
            'http://foo.com?param1=value1&param2={bar}'"
        );
    }

    #[test]
    fn url_with_single_quote() {
        let secrets: &[&str] = &[];
        let request = RequestSpec {
            method: Method("GET".to_string()),
            url: Url::from_str("http://127.0.0.1:8000/x'; touch POC_A_MARKER; : '").unwrap(),
            ..Default::default()
        };

        let context_dir = ContextDir::default();
        let cookie_store = CookieStore::new();
        let options = ClientOptions::default();
        let output = None;

        let cmd = CurlCmd::new(
            &request,
            &cookie_store,
            &context_dir,
            output.as_ref(),
            &options,
        );
        assert_eq!(
            cmd.to_redacted_string(secrets),
            "curl $'http://127.0.0.1:8000/x\\'; touch POC_A_MARKER; : \\''"
        );
    }

    #[test]
    fn query_request() {
        let secrets: &[&str] = &[];
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

        let context_dir = ContextDir::default();
        let cookie_store = CookieStore::new();
        let options = ClientOptions::default();
        let output = None;

        let cmd = CurlCmd::new(
            &request,
            &cookie_store,
            &context_dir,
            output.as_ref(),
            &options,
        );
        assert_eq!(
            cmd.to_redacted_string(secrets),
            "curl 'http://localhost:8000/querystring-params?param1=value1&param2=a%20b'",
        );

        // Add som query param in the URL
        request.url =
            Url::from_str("http://localhost:8000/querystring-params?param3=foo&param4=bar")
                .unwrap();
        let cmd = CurlCmd::new(
            &request,
            &cookie_store,
            &context_dir,
            output.as_ref(),
            &options,
        );
        assert_eq!(
            cmd.to_redacted_string(secrets),
            "curl 'http://localhost:8000/querystring-params?param3=foo&param4=bar&param1=value1&param2=a%20b'",
        );
    }

    #[test]
    fn form_request() {
        let secrets: &[&str] = &[];
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

        let context_dir = ContextDir::default();
        let cookie_store = CookieStore::new();
        let options = ClientOptions::default();
        let output = None;

        let cmd = CurlCmd::new(
            &request,
            &cookie_store,
            &context_dir,
            output.as_ref(),
            &options,
        );
        assert_eq!(
            cmd.to_redacted_string(secrets),
            "curl \
            --header 'Content-Type: application/x-www-form-urlencoded' \
            --data 'param1=value1' \
            --data 'param2=a%20b' \
            'http://localhost/form-params'"
        );
    }

    #[test]
    fn json_request() {
        let secrets: &[&str] = &[];
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

        let context_dir = ContextDir::default();
        let cookie_store = CookieStore::new();
        let options = ClientOptions::default();
        let output = None;

        let cmd = CurlCmd::new(
            &request,
            &cookie_store,
            &context_dir,
            output.as_ref(),
            &options,
        );
        assert_eq!(
            cmd.to_redacted_string(secrets),
            "curl \
            --request POST \
            --header 'content-type: application/vnd.api+json' \
            'http://localhost/json'"
        );

        // Add a non-empty body
        request.body = Body::Text("{\"foo\":\"bar\"}".to_string());
        let cmd = CurlCmd::new(
            &request,
            &cookie_store,
            &context_dir,
            output.as_ref(),
            &options,
        );
        assert_eq!(
            cmd.to_redacted_string(secrets),
            "curl \
            --header 'content-type: application/vnd.api+json' \
            --data '{\"foo\":\"bar\"}' \
            'http://localhost/json'"
        );

        // Change method
        request.method = Method("PUT".to_string());
        let cmd = CurlCmd::new(
            &request,
            &cookie_store,
            &context_dir,
            output.as_ref(),
            &options,
        );
        assert_eq!(
            cmd.to_redacted_string(secrets),
            "curl \
            --request PUT \
            --header 'content-type: application/vnd.api+json' \
            --data '{\"foo\":\"bar\"}' \
            'http://localhost/json'"
        );
    }

    #[test]
    fn post_binary_file() {
        let secrets: &[&str] = &[];
        let request = RequestSpec {
            method: Method("POST".to_string()),
            url: Url::from_str("http://localhost:8000/hello").unwrap(),
            body: Body::File(b"Hello World!".to_vec(), "foo.bin".to_string()),
            ..Default::default()
        };

        let context_dir = ContextDir::default();
        let cookie_store = CookieStore::new();
        let options = ClientOptions::default();
        let output = None;

        let cmd = CurlCmd::new(
            &request,
            &cookie_store,
            &context_dir,
            output.as_ref(),
            &options,
        );
        assert_eq!(
            cmd.to_redacted_string(secrets),
            "curl \
            --header 'Content-Type:' \
            --data-binary '@foo.bin' \
            'http://localhost:8000/hello'"
        );
    }

    #[test]
    fn post_binary_body_with_null_bytes() {
        let secrets: &[&str] = &[];
        // https://github.com/Orange-OpenSource/hurl/issues/5141
        // A body containing a NUL byte can not be embedded in a shell string:
        // the curl command must read the body from stdin instead.
        let mut request = RequestSpec {
            method: Method("POST".to_string()),
            url: Url::from_str("http://localhost:8000/post-bytes-null").unwrap(),
            body: Body::Binary(vec![0, 1, 2, 3]),
            ..Default::default()
        };
        let mut headers = HeaderVec::new();
        headers.push(Header::new("Content-Type", "application/octet-stream"));
        request.headers = headers;

        let context_dir = ContextDir::default();
        let cookie_store = CookieStore::new();
        let options = ClientOptions::default();
        let output = None;

        let cmd = CurlCmd::new(
            &request,
            &cookie_store,
            &context_dir,
            output.as_ref(),
            &options,
        );
        assert_eq!(
            cmd.to_redacted_string(secrets),
            "printf '\\x00\\x01\\x02\\x03' | \
            curl \
            --header 'Content-Type: application/octet-stream' \
            --data-binary @- \
            'http://localhost:8000/post-bytes-null'"
        );
    }

    #[test]
    fn post_binary_body_without_null_bytes() {
        let secrets: &[&str] = &[];
        // Control: without a NUL byte, the rendering is unchanged.
        let request = RequestSpec {
            method: Method("POST".to_string()),
            url: Url::from_str("http://localhost:8000/post-bytes").unwrap(),
            body: Body::Binary(vec![1, 2, 3]),
            ..Default::default()
        };

        let context_dir = ContextDir::default();
        let cookie_store = CookieStore::new();
        let options = ClientOptions::default();
        let output = None;

        let cmd = CurlCmd::new(
            &request,
            &cookie_store,
            &context_dir,
            output.as_ref(),
            &options,
        );
        assert_eq!(
            cmd.to_redacted_string(secrets),
            "curl \
            --header 'Content-Type: application/octet-stream' \
            --data $'\\x01\\x02\\x03' \
            'http://localhost:8000/post-bytes'"
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
            Header::new("Host", "example.com").curl_args("--header"),
            vec![Arg::plain("--header"), Arg::quoted("Host: example.com")]
        );
        assert_eq!(
            Header::new("If-Match", "\"e0023aa4e\"").curl_args("--proxy-header"),
            vec![
                Arg::plain("--proxy-header"),
                Arg::quoted("If-Match: \"e0023aa4e\"")
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
            Arg::quoted("hello")
        );

        if cfg!(unix) {
            assert_eq!(
                Body::File(vec![], "filename".to_string()).curl_arg(&context_dir),
                Arg::quoted("@/tmp/filename")
            );
        }

        assert_eq!(
            Body::Binary(vec![1, 2, 3]).curl_arg(&context_dir),
            Arg::Bytes(vec![1, 2, 3])
        );
    }

    #[test]
    fn test_encode_arg() {
        let secrets: &[&str] = &[];
        let encode = |arg: Arg| arg.encode(secrets);

        assert_eq!(encode(Arg::quoted("hello")), "'hello'");
        assert_eq!(encode(Arg::quoted("")), "''");
        assert_eq!(encode(Arg::quoted("\\n")), "'\\n'");
        assert_eq!(encode(Arg::quoted("'")), "$'\\''");
        assert_eq!(encode(Arg::quoted("\\'")), "$'\\\\\\''");
        assert_eq!(encode(Arg::quoted("\n")), "$'\\n'");
        assert_eq!(encode(Arg::quoted("\t")), "$'\\t'");

        assert_eq!(
            encode(Arg::plain("tests_ssl/certs/client/cert.pem:foobar")),
            "tests_ssl/certs/client/cert.pem:foobar"
        );
        assert_eq!(
            encode(Arg::plain("foo.com:80:localhost:8000")),
            "foo.com:80:localhost:8000"
        );
        assert_eq!(encode(Arg::plain("-")), "-");
        assert_eq!(encode(Arg::plain("")), "''");
        assert_eq!(encode(Arg::plain("my file")), "'my file'");
        assert_eq!(encode(Arg::plain("~/foo")), "'~/foo'");
        assert_eq!(
            encode(Arg::plain("foo.out; touch POC_A_MARKER")),
            "'foo.out; touch POC_A_MARKER'"
        );
        assert_eq!(encode(Arg::plain("$(touch x)")), "'$(touch x)'");
        assert_eq!(encode(Arg::plain("a'b")), "$'a\\'b'");

        assert_eq!(encode(Arg::Bytes(vec![1, 2, 3])), "$'\\x01\\x02\\x03'");
    }

    #[test]
    fn test_encode_arg_with_secrets() {
        let secrets = ["s3cr'et"];
        assert_eq!(
            Arg::quoted("Token: s3cr'et").encode(&secrets),
            "'Token: ***'"
        );
        assert_eq!(Arg::plain("foo-s3cr'et").encode(&secrets), "'foo-***'");
        assert_eq!(Arg::plain("--header").encode(&secrets), "--header");
    }

    #[test]
    fn redacted_curl_cmd() {
        // A secret containing a quote is encoded in the command line, so it must be redacted
        // before being encoded.
        let mut headers = HeaderVec::new();
        headers.push(Header::new("Authorization", "Bearer s3cr'et"));
        let request = RequestSpec {
            method: Method("GET".to_string()),
            url: Url::from_str("http://localhost:8000/hello?token=s3cr'et").unwrap(),
            headers,
            ..Default::default()
        };

        let context_dir = ContextDir::default();
        let cookie_store = CookieStore::new();
        let options = ClientOptions::default();

        let cmd = CurlCmd::new(&request, &cookie_store, &context_dir, None, &options);
        let secrets: &[&str] = &[];
        assert_eq!(
            cmd.to_redacted_string(secrets),
            "curl \
            --header $'Authorization: Bearer s3cr\\'et' \
            $'http://localhost:8000/hello?token=s3cr\\'et'"
        );
        let secrets = &["s3cr'et"];
        assert_eq!(
            cmd.to_redacted_string(secrets),
            "curl \
            --header 'Authorization: Bearer ***' \
            'http://localhost:8000/hello?token=***'"
        );
    }

    #[test]
    fn cookie_with_single_quote() {
        let secrets: &[&str] = &[];
        let request = RequestSpec {
            method: Method("GET".to_string()),
            url: Url::from_str("http://localhost:8000/hello").unwrap(),
            cookies: vec![crate::http::RequestCookie {
                name: "cookie1".to_string(),
                value: "value'with'quotes".to_string(),
            }],
            ..Default::default()
        };

        let context_dir = ContextDir::default();
        let cookie_store = CookieStore::new();
        let options = ClientOptions::default();
        let output = None;

        let cmd = CurlCmd::new(
            &request,
            &cookie_store,
            &context_dir,
            output.as_ref(),
            &options,
        );
        assert_eq!(
            cmd.to_redacted_string(secrets),
            "curl --cookie $'cookie1=value\\'with\\'quotes' 'http://localhost:8000/hello'"
        );
    }
}
