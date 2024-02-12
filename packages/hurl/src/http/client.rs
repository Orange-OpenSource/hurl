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
use std::str;
use std::str::FromStr;

use base64::engine::general_purpose;
use base64::Engine;
use chrono::Utc;
use curl::easy::{List, NetRc, SslOpt};
use curl::{easy, Version};
use encoding::all::ISO_8859_1;
use encoding::{DecoderTrap, Encoding};
use url::Url;

use crate::http::certificate::Certificate;
use crate::http::core::*;
use crate::http::header::{
    HeaderVec, ACCEPT_ENCODING, AUTHORIZATION, CONTENT_TYPE, EXPECT, LOCATION, USER_AGENT,
};
use crate::http::options::ClientOptions;
use crate::http::request::*;
use crate::http::request_spec::*;
use crate::http::response::*;
use crate::http::timings::Timings;
use crate::http::{easy_ext, Call, Header, HttpError, Verbosity};
use crate::runner::Output;
use crate::util::logger::Logger;
use crate::util::path::ContextDir;

/// Defines an HTTP client to execute HTTP requests.
///
/// Most of the methods are delegated to libcurl functions, while some
/// features are implemented "by hand" (like retry, redirection etc...)
#[derive(Debug)]
pub struct Client {
    /// The handle to libcurl binding
    handle: Box<easy::Easy>,
    /// Current State
    state: ClientState,
    /// HTTP version support
    http2: bool,
    http3: bool,
}

/// Represents the state of the HTTP client.
/// We only keep the last requested HTTP version because it's the only property which change
/// must trigger a connection reset on the libcurl handle. The state can be queried to check
/// if there has been change from the previous HTTP request.
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
struct ClientState {
    changed: bool,
    requested_http_version: Option<RequestedHttpVersion>,
}

impl ClientState {
    /// Set a new requested HTTP version.
    pub fn set_requested_http_version(&mut self, version: RequestedHttpVersion) {
        if let Some(prev_version) = self.requested_http_version {
            self.changed = prev_version != version;
        }
        self.requested_http_version = Some(version);
    }

    /// Returns true if state has changed from the previous request.
    pub fn has_changed(&self) -> bool {
        self.changed
    }
}

impl Client {
    /// Creates HTTP Hurl client.
    pub fn new() -> Client {
        let handle = easy::Easy::new();
        let version = Version::get();
        Client {
            handle: Box::new(handle),
            state: ClientState::default(),
            http2: version.feature_http2(),
            http3: version.feature_http3(),
        }
    }

    /// Executes an HTTP request `request_spec`, optionally follows redirection and returns a list of [`Call`].
    pub fn execute_with_redirect(
        &mut self,
        request_spec: &RequestSpec,
        options: &ClientOptions,
        logger: &Logger,
    ) -> Result<Vec<Call>, HttpError> {
        let mut calls = vec![];

        let mut request_spec = request_spec.clone();

        // Unfortunately, follow-location feature from libcurl can not be used
        // libcurl returns a single list of headers for the 2 responses
        // Hurl needs to keep everything.
        let mut redirect_count = 0;
        loop {
            let call = self.execute(&request_spec, options, logger)?;
            let base_url = call.request.base_url()?;
            let redirect_url = self.get_follow_location(&call.response, &base_url);
            let status = call.response.status;
            calls.push(call);
            if !options.follow_location || redirect_url.is_none() {
                break;
            }
            let redirect_url = redirect_url.unwrap();
            logger.debug("");
            logger.debug(format!("=> Redirect to {redirect_url}").as_str());
            logger.debug("");
            redirect_count += 1;
            if let Some(max_redirect) = options.max_redirect {
                if redirect_count > max_redirect {
                    return Err(HttpError::TooManyRedirect);
                }
            }
            let redirect_method = get_redirect_method(status, request_spec.method);
            let headers = if options.follow_location_trusted {
                request_spec.headers
            } else {
                request_spec
                    .headers
                    .retain(|h| h.name.to_lowercase() != AUTHORIZATION.to_lowercase());
                request_spec.headers
            };
            request_spec = RequestSpec {
                method: redirect_method,
                url: redirect_url,
                headers,
                ..Default::default()
            };
        }
        Ok(calls)
    }

    /// Executes an HTTP request `request_spec`, without following redirection and returns a
    /// pair of [`Call`].
    pub fn execute(
        &mut self,
        request_spec: &RequestSpec,
        options: &ClientOptions,
        logger: &Logger,
    ) -> Result<Call, HttpError> {
        // The handle can be mutated in this function: to start from a clean state, we reset it
        // prior to everything.
        self.handle.reset();

        let (url, method) = self.configure(request_spec, options, logger)?;

        let start = Utc::now();
        let verbose = options.verbosity.is_some();
        let very_verbose = options.verbosity == Some(Verbosity::VeryVerbose);
        let mut request_headers = HeaderVec::new();
        let mut status_lines = vec![];
        let mut response_headers = vec![];
        let has_body_data = !request_spec.body.bytes().is_empty()
            || !request_spec.form.is_empty()
            || !request_spec.multipart.is_empty();

        // `request_body` are request body bytes computed by libcurl (the real bytes sent over the wire)
        // whereas`request_spec_body` are request body bytes provided by Hurl user. For instance, if user uses
        // a [FormParam] section, `request_body` is empty whereas libcurl sent a url-form encoded list
        // of key-value.
        let mut request_body = Vec::<u8>::new();
        let mut response_body = Vec::<u8>::new();

        {
            let mut transfer = self.handle.transfer();

            transfer.debug_function(|info_type, data| match info_type {
                // Return all request headers (not one by one)
                easy::InfoType::HeaderOut => {
                    let mut lines = split_lines(data);
                    logger.debug_method_version_out(&lines[0]);

                    // Extracts request headers from libcurl debug info.
                    lines.pop().unwrap(); // Remove last empty line.
                    lines.remove(0); // Remove method/path/version line.
                    for line in lines {
                        if let Some(header) = Header::parse(&line) {
                            request_headers.push(header);
                        }
                    }

                    // If we don't send any data, we log headers and empty body here
                    // instead of relying on libcurl computing body in `easy::InfoType::DataOut`.
                    // because libcurl dont call `easy::InfoType::DataOut` if there is no data
                    // to send.
                    if !has_body_data && verbose {
                        let debug_request = Request::new(
                            &method.to_string(),
                            &url,
                            request_headers.clone(),
                            vec![],
                        );
                        for header in &debug_request.headers {
                            logger.debug_header_out(&header.name, &header.value);
                        }
                        logger.info(">");

                        if very_verbose {
                            debug_request.log_body(true, logger);
                        }
                    }
                }
                // We use this callback to get the real body bytes sent by libcurl.
                easy::InfoType::DataOut => {
                    // We log request headers with `easy::InfoType::DataOut` using libcurl
                    // debug functions: there is no libcurl function to get the request headers.
                    // As we can be called multiple times in this callback, we only log headers the
                    // first time we send data (marked when the request body is empty).
                    if verbose && request_body.is_empty() {
                        let debug_request = Request::new(
                            &method.to_string(),
                            &url,
                            request_headers.clone(),
                            Vec::from(data),
                        );
                        for header in &debug_request.headers {
                            logger.debug_header_out(&header.name, &header.value);
                        }
                        logger.info(">");

                        if very_verbose {
                            debug_request.log_body(true, logger);
                        }
                    }

                    // Extracts request body from libcurl debug info.
                    request_body.extend(data);
                }
                // Curl debug logs
                easy::InfoType::Text => {
                    let len = data.len();
                    if very_verbose && len > 0 {
                        let text = str::from_utf8(&data[..len - 1]);
                        if let Ok(text) = text {
                            logger.debug_curl(text);
                        }
                    }
                }
                _ => {}
            })?;
            transfer.header_function(|h| {
                if let Some(s) = decode_header(h) {
                    if s.starts_with("HTTP/") {
                        status_lines.push(s);
                    } else {
                        response_headers.push(s)
                    }
                }
                true
            })?;

            transfer.write_function(|data| {
                response_body.extend(data);
                Ok(data.len())
            })?;

            if let Err(e) = transfer.perform() {
                let code = e.code() as i32; // due to windows build
                let description = match e.extra_description() {
                    None => e.description().to_string(),
                    Some(s) => s.to_string(),
                };
                return Err(HttpError::Libcurl { code, description });
            }
        }

        let status = self.handle.response_code()?;
        // TODO: explain why status_lines is Vec ?
        let version = match status_lines.last() {
            None => return Err(HttpError::StatuslineIsMissing),
            Some(status_line) => self.parse_response_version(status_line)?,
        };
        let headers = self.parse_response_headers(&response_headers);
        let length = response_body.len();
        let certificate = if let Some(cert_info) = easy_ext::get_certinfo(&self.handle)? {
            match Certificate::try_from(cert_info) {
                Ok(value) => Some(value),
                Err(message) => {
                    logger.error(format!("can not parse certificate - {message}").as_str());
                    None
                }
            }
        } else {
            None
        };
        let stop = Utc::now();
        let duration = (stop - start).to_std().unwrap();
        let timings = Timings::new(&mut self.handle, start, stop);

        let request = Request::new(&method.to_string(), &url, request_headers, request_body);
        let response = Response::new(
            version,
            status,
            headers,
            response_body,
            duration,
            &url,
            certificate,
        );

        if verbose {
            // FIXME: the cast to u64 seems not necessary.
            //  If we dont cast from u128 and try to format! or println!
            //  we have a segfault on Alpine Docker images and Rust 1.68.0, whereas it was
            //  ok with Rust >= 1.67.0.
            let duration = duration.as_millis() as u64;
            logger.debug_important(
                format!("Response: (received {length} bytes in {duration} ms)").as_str(),
            );
            logger.debug("");

            // FIXME: Explain why there may be multiple status line
            status_lines
                .iter()
                .filter(|s| s.starts_with("HTTP/"))
                .for_each(|s| logger.debug_status_version_in(s.trim()));

            for header in &response.headers {
                logger.debug_header_in(&header.name, &header.value);
            }
            logger.info("<");
            if very_verbose {
                response.log_body(true, logger);
                logger.debug("");
                timings.log(logger);
            }
        }

        Ok(Call {
            request,
            response,
            timings,
        })
    }

    /// Configure libcurl handle to send a `request_spec`, using `options`.
    /// If configuration is successful, returns a tuple of the concrete requested URL and method.
    fn configure(
        &mut self,
        request_spec: &RequestSpec,
        options: &ClientOptions,
        logger: &Logger,
    ) -> Result<(String, Method), HttpError> {
        // Activates cookie engine.
        // See <https://curl.se/libcurl/c/CURLOPT_COOKIEFILE.html>
        // > It also enables the cookie engine, making libcurl parse and send cookies on subsequent
        // > requests with this handle.
        // > By passing the empty string ("") to this option, you enable the cookie
        // > engine without reading any initial cookies.
        self.handle
            .cookie_file(options.cookie_input_file.clone().unwrap_or_default())
            .unwrap();

        // We force libcurl verbose mode regardless of Hurl verbose option to be able
        // to capture HTTP request headers in libcurl `debug_function`. That's the only
        // way to get access to the outgoing headers.
        self.handle.verbose(true)?;

        // We checks libcurl HTTP version support.
        let http_version = options.http_version;
        if (http_version == RequestedHttpVersion::Http2 && !self.http2)
            || (http_version == RequestedHttpVersion::Http3 && !self.http3)
        {
            return Err(HttpError::UnsupportedHttpVersion(http_version));
        }

        // libcurl tries to reuse connections as much as possible (see <https://curl.se/libcurl/c/CURLOPT_HTTP_VERSION.html>)
        // That's why an `handle` initiated with a HTTP 2 version may keep using HTTP 2 protocol
        // even if we ask to switch to HTTP 3 in the same session (using `[Options]` section for
        // instance).
        // > Note that the HTTP version is just a request. libcurl still prioritizes to reuse
        // > existing connections so it might then reuse a connection using a HTTP version you
        // > have not asked for.
        //
        // So, if we detect a change of requested HTTP version, we force libcurl to refresh its
        // connections (see <https://curl.se/libcurl/c/CURLOPT_FRESH_CONNECT.html>)
        self.state.set_requested_http_version(http_version);
        if self.state.has_changed() {
            logger.debug("Force refreshing connections because requested HTTP version change");
            self.handle.fresh_connect(true)?;
        }
        self.handle.http_version(options.http_version.into())?;

        self.handle.ip_resolve(options.ip_resolve.into())?;

        // Activates the access of certificates info chain after a transfer has been executed.
        self.handle.certinfo(true)?;

        if !options.connects_to.is_empty() {
            let connects = to_list(&options.connects_to);
            self.handle.connect_to(connects)?;
        }
        if !options.resolves.is_empty() {
            let resolves = to_list(&options.resolves);
            self.handle.resolve(resolves)?;
        }
        self.handle.ssl_verify_host(!options.insecure)?;
        self.handle.ssl_verify_peer(!options.insecure)?;
        if let Some(cacert_file) = options.cacert_file.clone() {
            self.handle.cainfo(cacert_file)?;
            self.handle.ssl_cert_type("PEM")?;
        }
        if let Some(client_cert_file) = &options.client_cert_file {
            match parse_cert_password(client_cert_file) {
                (cert, Some(password)) => {
                    self.handle.ssl_cert(cert)?;
                    self.handle.key_password(&password)?;
                }
                (cert, None) => {
                    self.handle.ssl_cert(cert)?;
                }
            }
            self.handle.ssl_cert_type("PEM")?;
        }
        if let Some(client_key_file) = options.client_key_file.clone() {
            self.handle.ssl_key(client_key_file)?;
            self.handle.ssl_cert_type("PEM")?;
        }
        self.handle.path_as_is(options.path_as_is)?;
        if let Some(proxy) = options.proxy.clone() {
            self.handle.proxy(proxy.as_str())?;
        }
        if let Some(s) = options.no_proxy.clone() {
            self.handle.noproxy(s.as_str())?;
        }
        if let Some(unix_socket) = &options.unix_socket {
            self.handle.unix_socket(unix_socket)?;
        }
        if let Some(filename) = &options.netrc_file {
            easy_ext::netrc_file(&mut self.handle, filename)?;
            self.handle.netrc(if options.netrc_optional {
                NetRc::Optional
            } else {
                NetRc::Required
            })?;
        } else if options.netrc_optional {
            self.handle.netrc(NetRc::Optional)?;
        } else if options.netrc {
            self.handle.netrc(NetRc::Required)?;
        }
        self.handle.timeout(options.timeout)?;
        self.handle.connect_timeout(options.connect_timeout)?;

        self.set_ssl_options(options.ssl_no_revoke)?;

        let url = self.generate_url(&request_spec.url, &request_spec.querystring);
        self.handle.url(url.as_str())?;
        let method = &request_spec.method;
        self.set_method(method)?;
        self.set_cookies(&request_spec.cookies)?;
        self.set_form(&request_spec.form)?;
        self.set_multipart(&request_spec.multipart)?;
        let request_spec_body = &request_spec.body.bytes();
        self.set_body(request_spec_body)?;
        self.set_headers(request_spec, options)?;
        if let Some(aws_sigv4) = &options.aws_sigv4 {
            if let Err(e) = self.handle.aws_sigv4(aws_sigv4.as_str()) {
                return match e.code() {
                    curl_sys::CURLE_UNKNOWN_OPTION => Err(HttpError::LibcurlUnknownOption {
                        option: "aws-sigv4".to_string(),
                        minimum_version: "7.75.0".to_string(),
                    }),
                    _ => Err(e.into()),
                };
            }
        }
        if *method == Method("HEAD".to_string()) {
            self.handle.nobody(true)?;
        }
        Ok((url, method.clone()))
    }

    /// Generates URL.
    fn generate_url(&mut self, url: &str, params: &[Param]) -> String {
        if params.is_empty() {
            url.to_string()
        } else {
            let url = if url.ends_with('?') {
                url.to_string()
            } else if url.contains('?') {
                format!("{url}&")
            } else {
                format!("{url}?")
            };
            let s = self.url_encode_params(params);
            format!("{url}{s}")
        }
    }

    /// Sets HTTP method.
    fn set_method(&mut self, method: &Method) -> Result<(), HttpError> {
        self.handle.custom_request(method.to_string().as_str())?;
        Ok(())
    }

    /// Sets HTTP headers.
    fn set_headers(
        &mut self,
        request_spec: &RequestSpec,
        options: &ClientOptions,
    ) -> Result<(), HttpError> {
        let mut list = List::new();

        for header in &request_spec.headers {
            list.append(&format!("{}: {}", header.name, header.value))?;
        }

        // If request has no Content-Type header, we set it if the content type has been set
        // implicitly on this request.
        if !request_spec.headers.contains_key(CONTENT_TYPE) {
            if let Some(s) = &request_spec.implicit_content_type {
                list.append(&format!("{}: {s}", CONTENT_TYPE))?;
            } else {
                // We remove default Content-Type headers added by curl because we want
                // to explicitly manage this header.
                // For instance, with --data option, curl will send a 'Content-type: application/x-www-form-urlencoded'
                // header.
                list.append(&format!("{}:", CONTENT_TYPE))?;
            }
        }

        // Workaround for libcurl issue #11664: When Hurl explicitly sets `Expect:` to remove the header,
        // libcurl will generate `SignedHeaders` that include `expect` even though the header is not
        // present, causing some APIs to reject the request.
        // Therefore we only remove this header when not in aws_sigv4 mode.
        if !request_spec.headers.contains_key(EXPECT) && options.aws_sigv4.is_none() {
            // We remove default Expect headers added by curl because we want
            // to explicitly manage this header.
            list.append(&format!("{}:", EXPECT))?;
        }

        if !request_spec.headers.contains_key(USER_AGENT) {
            let user_agent = match options.user_agent {
                Some(ref u) => u.clone(),
                None => format!("hurl/{}", clap::crate_version!()),
            };
            list.append(&format!("{}: {user_agent}", USER_AGENT))?;
        }

        if let Some(user) = &options.user {
            if options.aws_sigv4.is_some() {
                // curl's aws_sigv4 support needs to know the username and password for the
                // request, as it uses those values to calculate the Authorization header for the
                // AWS V4 signature.
                if let Some((username, password)) = user.split_once(':') {
                    self.handle.username(username)?;
                    self.handle.password(password)?;
                }
            } else {
                let user = user.as_bytes();
                let authorization = general_purpose::STANDARD.encode(user);
                if !request_spec.headers.contains_key(AUTHORIZATION) {
                    list.append(&format!("{}: Basic {authorization}", AUTHORIZATION))?;
                }
            }
        }
        if options.compressed && !request_spec.headers.contains_key(ACCEPT_ENCODING) {
            list.append(&format!("{}: gzip, deflate, br", ACCEPT_ENCODING))?;
        }

        self.handle.http_headers(list)?;
        Ok(())
    }

    /// Sets request cookies.
    fn set_cookies(&mut self, cookies: &[RequestCookie]) -> Result<(), HttpError> {
        let s = cookies
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join("; ");
        if !s.is_empty() {
            self.handle.cookie(s.as_str())?;
        }
        Ok(())
    }

    /// Sets form params.
    fn set_form(&mut self, params: &[Param]) -> Result<(), HttpError> {
        if !params.is_empty() {
            let s = self.url_encode_params(params);
            self.handle.post_fields_copy(s.as_bytes())?;
        }
        Ok(())
    }

    /// Sets multipart form data.
    fn set_multipart(&mut self, params: &[MultipartParam]) -> Result<(), HttpError> {
        if !params.is_empty() {
            let mut form = easy::Form::new();
            for param in params {
                // TODO: we could remove these `unwrap` if we implement conversion
                // from libcurl::FormError to HttpError
                match param {
                    MultipartParam::Param(Param { name, value }) => {
                        form.part(name).contents(value.as_bytes()).add().unwrap()
                    }
                    MultipartParam::FileParam(FileParam {
                        name,
                        filename,
                        data,
                        content_type,
                    }) => form
                        .part(name)
                        .buffer(filename, data.clone())
                        .content_type(content_type)
                        .add()
                        .unwrap(),
                }
            }
            self.handle.httppost(form)?;
        }
        Ok(())
    }

    /// Sets request body.
    fn set_body(&mut self, data: &[u8]) -> Result<(), HttpError> {
        if !data.is_empty() {
            self.handle.post(true)?;
            self.handle.post_fields_copy(data)?;
        }
        Ok(())
    }

    /// Sets SSL options
    fn set_ssl_options(&mut self, no_revoke: bool) -> Result<(), HttpError> {
        let mut ssl_opt = SslOpt::new();
        ssl_opt.no_revoke(no_revoke);
        self.handle.ssl_options(&ssl_opt)?;
        Ok(())
    }

    /// URL encodes parameters.
    fn url_encode_params(&mut self, params: &[Param]) -> String {
        params
            .iter()
            .map(|p| {
                let value = self.handle.url_encode(p.value.as_bytes());
                format!("{}={}", p.name, value)
            })
            .collect::<Vec<String>>()
            .join("&")
    }

    /// Parses HTTP response version.
    fn parse_response_version(&mut self, line: &str) -> Result<HttpVersion, HttpError> {
        if line.starts_with("HTTP/1.0") {
            Ok(HttpVersion::Http10)
        } else if line.starts_with("HTTP/1.1") {
            Ok(HttpVersion::Http11)
        } else if line.starts_with("HTTP/2") {
            Ok(HttpVersion::Http2)
        } else if line.starts_with("HTTP/3") {
            Ok(HttpVersion::Http3)
        } else {
            Err(HttpError::CouldNotParseResponse)
        }
    }

    /// Parse headers from libcurl responses.
    fn parse_response_headers(&mut self, lines: &[String]) -> HeaderVec {
        let mut headers = HeaderVec::new();
        for line in lines {
            if let Some(header) = Header::parse(line) {
                headers.push(header);
            }
        }
        headers
    }

    /// Retrieves an optional location to follow
    ///
    /// You need:
    /// 1. the option follow_location set to true
    /// 2. a 3xx response code
    /// 3. a header Location
    fn get_follow_location(&mut self, response: &Response, base_url: &str) -> Option<String> {
        let response_code = response.status;
        if !(300..400).contains(&response_code) {
            return None;
        }
        response
            .headers
            .get(LOCATION)
            .map(|h| get_redirect_url(&h.value, base_url))
    }

    /// Returns cookie storage.
    pub fn get_cookie_storage(&mut self) -> Vec<Cookie> {
        let list = self.handle.cookies().unwrap();
        let mut cookies = vec![];
        for cookie in list.iter() {
            let line = str::from_utf8(cookie).unwrap();
            if let Ok(cookie) = Cookie::from_str(line) {
                cookies.push(cookie);
            } else {
                eprintln!("warning: line <{line}> can not be parsed as cookie");
            }
        }
        cookies
    }

    /// Adds a cookie to the cookie jar.
    pub fn add_cookie(&mut self, cookie: &Cookie, options: &ClientOptions) {
        if options.verbosity.is_some() {
            eprintln!("* add to cookie store: {cookie}");
        }
        self.handle
            .cookie_list(cookie.to_string().as_str())
            .unwrap();
    }

    /// Clears cookie storage.
    pub fn clear_cookie_storage(&mut self, options: &ClientOptions) {
        if options.verbosity.is_some() {
            eprintln!("* clear cookie storage");
        }
        self.handle.cookie_list("ALL").unwrap();
    }

    /// Returns curl command-line for the HTTP `request_spec` run by this client.
    pub fn curl_command_line(
        &mut self,
        request_spec: &RequestSpec,
        context_dir: &ContextDir,
        output: Option<&Output>,
        options: &ClientOptions,
    ) -> String {
        let mut arguments = vec!["curl".to_string()];
        arguments.append(&mut request_spec.curl_args(context_dir));

        // We extract the last part of the arguments (the url) to insert it
        // after all the options
        let url = arguments.pop().unwrap();

        let cookies = all_cookies(&self.get_cookie_storage(), request_spec);
        if !cookies.is_empty() {
            arguments.push("--cookie".to_string());
            arguments.push(format!(
                "'{}'",
                cookies
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>()
                    .join("; ")
            ));
        }
        arguments.append(&mut options.curl_args());

        // --output is not an option of the HTTP client, we deal with it here:
        if let Some(output) = output {
            arguments.push("--output".to_string());
            arguments.push(output.to_string());
        }

        arguments.push(url);
        arguments.join(" ")
    }
}

/// Returns the redirect url.
fn get_redirect_url(location: &str, base_url: &str) -> String {
    if location.starts_with('/') {
        format!("{base_url}{location}")
    } else {
        location.to_string()
    }
}

/// Returns the method used for redirecting a request/response with `response_status`.
fn get_redirect_method(response_status: u32, original_method: Method) -> Method {
    // This replicates curl's behavior
    match response_status {
        301..=303 => Method("GET".to_string()),
        // Could be only 307 and 308, but curl does this for all 3xx
        // codes not converted to GET above.
        _ => original_method,
    }
}

/// Returns cookies from both cookies from the cookie storage and the request.
pub fn all_cookies(cookie_storage: &[Cookie], request_spec: &RequestSpec) -> Vec<RequestCookie> {
    let mut cookies = request_spec.cookies.clone();
    cookies.append(
        &mut cookie_storage
            .iter()
            .filter(|c| c.expires != "1") // cookie expired when libcurl set value to 1?
            .filter(|c| match_cookie(c, request_spec.url.as_str()))
            .map(|c| RequestCookie {
                name: c.name.clone(),
                value: c.value.clone(),
            })
            .collect(),
    );
    cookies
}

/// Matches cookie for a given URL.
pub fn match_cookie(cookie: &Cookie, url: &str) -> bool {
    // FIXME: is it possible to do it with libcurl?
    let url = match Url::parse(url) {
        Ok(url) => url,
        Err(_) => return false,
    };
    if let Some(domain) = url.domain() {
        if cookie.include_subdomain == "FALSE" {
            if cookie.domain != domain {
                return false;
            }
        } else if !domain.ends_with(cookie.domain.as_str()) {
            return false;
        }
    }
    url.path().starts_with(cookie.path.as_str())
}

impl Header {
    /// Parses an HTTP header line received from the server
    /// It does not panic. Just returns `None` if it can not be parsed.
    pub fn parse(line: &str) -> Option<Header> {
        match line.find(':') {
            Some(index) => {
                let (name, value) = line.split_at(index);
                Some(Header::new(name.trim(), value[1..].trim()))
            }
            None => None,
        }
    }
}

/// Splits an array of bytes into HTTP lines (\r\n separator).
fn split_lines(data: &[u8]) -> Vec<String> {
    let mut lines = vec![];
    let mut start = 0;
    let mut i = 0;
    while i < (data.len() - 1) {
        if data[i] == 13 && data[i + 1] == 10 {
            if let Ok(s) = str::from_utf8(&data[start..i]) {
                lines.push(s.to_string());
            }
            start = i + 2;
            i += 2;
        } else {
            i += 1;
        }
    }
    lines
}

/// Decodes optionally header value as text with UTF-8 or ISO-8859-1 encoding.
pub fn decode_header(data: &[u8]) -> Option<String> {
    match str::from_utf8(data) {
        Ok(s) => Some(s.to_string()),
        Err(_) => match ISO_8859_1.decode(data, DecoderTrap::Strict) {
            Ok(s) => Some(s),
            Err(_) => {
                println!("Error decoding header both UTF-8 and ISO-8859-1 {data:?}");
                None
            }
        },
    }
}

/// Converts a list of [`String`] to a libcurl's list of strings.
fn to_list(items: &[String]) -> List {
    let mut list = List::new();
    items.iter().for_each(|l| list.append(l).unwrap());
    list
}

/// Parses a cert file name, with a potential user provided password, and returns a pair of
/// cert file name, password.
/// See <https://curl.se/docs/manpage.html#-E>
/// > In the <certificate> portion of the argument, you must escape the character ":" as "\:" so
/// > that it is not recognized as the password delimiter. Similarly, you must escape the character
/// > "\" as "\\" so that it is not recognized as an escape character.
fn parse_cert_password(cert_and_pass: &str) -> (String, Option<String>) {
    let mut iter = cert_and_pass.chars();
    let mut cert = String::new();
    let mut password = String::new();
    // The state of the parser:
    // - `true` if we're parsing the certificate portion of `cert_and_pass`
    // - `false` if we're parsing the password portion of `cert_and_pass`
    let mut parse_cert = true;
    while let Some(c) = iter.next() {
        if parse_cert {
            // We're parsing the certificate, do some escaping
            match c {
                '\\' => {
                    // We read the next escaped char, if we failed, we're at the end of this string,
                    // the read char is not an escaping \.
                    match iter.next() {
                        Some(c) => cert.push(c),
                        None => {
                            cert.push('\\');
                            break;
                        }
                    }
                }
                ':' if parse_cert => parse_cert = false,
                c => cert.push(c),
            }
        } else {
            // We have already found a cert/password separator, we don't need to escape anything now
            // we just update the password
            password.push(c);
        }
    }

    if parse_cert {
        (cert, None)
    } else {
        (cert, Some(password))
    }
}

impl From<RequestedHttpVersion> for easy::HttpVersion {
    fn from(value: RequestedHttpVersion) -> Self {
        match value {
            RequestedHttpVersion::Default => easy::HttpVersion::Any,
            RequestedHttpVersion::Http10 => easy::HttpVersion::V10,
            RequestedHttpVersion::Http11 => easy::HttpVersion::V11,
            RequestedHttpVersion::Http2 => easy::HttpVersion::V2,
            RequestedHttpVersion::Http3 => easy::HttpVersion::V3,
        }
    }
}

impl From<IpResolve> for easy::IpResolve {
    fn from(value: IpResolve) -> Self {
        match value {
            IpResolve::Default => easy::IpResolve::Any,
            IpResolve::IpV4 => easy::IpResolve::V4,
            IpResolve::IpV6 => easy::IpResolve::V6,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::default::Default;

    #[test]
    fn test_parse_header() {
        assert_eq!(
            Header::parse("Foo: Bar\r\n").unwrap(),
            Header::new("Foo", "Bar")
        );
        assert_eq!(
            Header::parse("Location: http://localhost:8000/redirected\r\n").unwrap(),
            Header::new("Location", "http://localhost:8000/redirected")
        );
        assert!(Header::parse("Foo").is_none());
    }

    #[test]
    fn test_split_lines_header() {
        let data = b"GET /hello HTTP/1.1\r\nHost: localhost:8000\r\n\r\n";
        let lines = split_lines(data);
        assert_eq!(lines.len(), 3);
        assert_eq!(lines.first().unwrap().as_str(), "GET /hello HTTP/1.1");
        assert_eq!(lines.get(1).unwrap().as_str(), "Host: localhost:8000");
        assert_eq!(lines.get(2).unwrap().as_str(), "");
    }

    #[test]
    fn test_match_cookie() {
        let cookie = Cookie {
            domain: "example.com".to_string(),
            include_subdomain: "FALSE".to_string(),
            path: "/".to_string(),
            https: String::new(),
            expires: String::new(),
            name: String::new(),
            value: String::new(),
            http_only: false,
        };
        assert!(match_cookie(&cookie, "http://example.com/toto"));
        assert!(!match_cookie(&cookie, "http://sub.example.com/tata"));
        assert!(!match_cookie(&cookie, "http://toto/tata"));

        let cookie = Cookie {
            domain: "example.com".to_string(),
            include_subdomain: "TRUE".to_string(),
            path: "/toto".to_string(),
            https: String::new(),
            expires: String::new(),
            name: String::new(),
            value: String::new(),
            http_only: false,
        };
        assert!(match_cookie(&cookie, "http://example.com/toto"));
        assert!(match_cookie(&cookie, "http://sub.example.com/toto"));
        assert!(!match_cookie(&cookie, "http://example.com/tata"));
    }

    #[test]
    fn test_redirect_url() {
        assert_eq!(
            get_redirect_url("http://localhost:8000/redirected", "http://localhost:8000"),
            "http://localhost:8000/redirected".to_string()
        );
        assert_eq!(
            get_redirect_url("/redirected", "http://localhost:8000"),
            "http://localhost:8000/redirected".to_string()
        );
    }

    #[test]
    fn test_redirect_method() {
        // Status of the response to be redirected | method of the original request | method of the new request
        let data = [
            (301, "GET", "GET"),
            (301, "POST", "GET"),
            (301, "DELETE", "GET"),
            (302, "GET", "GET"),
            (302, "POST", "GET"),
            (302, "DELETE", "GET"),
            (303, "GET", "GET"),
            (303, "POST", "GET"),
            (303, "DELETE", "GET"),
            (304, "GET", "GET"),
            (304, "POST", "POST"),
            (304, "DELETE", "DELETE"),
            (308, "GET", "GET"),
            (308, "POST", "POST"),
            (308, "DELETE", "DELETE"),
        ];
        for (status, original, redirected) in data {
            assert_eq!(
                get_redirect_method(status, Method(original.to_string())),
                Method(redirected.to_string())
            );
        }
    }

    #[test]
    fn http_client_state_always_http2() {
        let mut state = ClientState::default();
        assert!(!state.has_changed());

        // Client set HTTP 2 on all request, client state never changed

        // - => HTTP/2: no change
        state.set_requested_http_version(RequestedHttpVersion::Http2);
        assert!(!state.has_changed());

        // HTTP/2 => HTTP/2: no change
        state.set_requested_http_version(RequestedHttpVersion::Http2);
        assert!(!state.has_changed());
    }

    #[test]
    fn http_client_state_always_default() {
        let mut state = ClientState::default();
        assert!(!state.has_changed());

        // Client doesn't set HTTP version, client state never changed

        // - => Default: no change
        state.set_requested_http_version(RequestedHttpVersion::Default);
        assert!(!state.has_changed());

        // Default => Default: no change
        state.set_requested_http_version(RequestedHttpVersion::Default);
        assert!(!state.has_changed());
    }

    #[test]
    fn http_client_state_changes() {
        let mut state = ClientState::default();
        assert!(!state.has_changed());

        // Client set HTTP 2 on all request, client state never changed

        // - => HTTP/2: no change
        state.set_requested_http_version(RequestedHttpVersion::Http2);
        assert!(!state.has_changed());

        // HTTP/2 => HTTP/1.1: change
        state.set_requested_http_version(RequestedHttpVersion::Http11);
        assert!(state.has_changed());

        // HTTP/1.1 => HTTP/1.1: no change
        state.set_requested_http_version(RequestedHttpVersion::Http11);
        assert!(!state.has_changed());

        // HTTP/1.1 => Default: change
        state.set_requested_http_version(RequestedHttpVersion::Default);
        assert!(state.has_changed());

        // Default => Default: no change
        state.set_requested_http_version(RequestedHttpVersion::Default);
        assert!(!state.has_changed());
    }

    #[test]
    fn command_line_args() {
        let mut client = Client::new();
        let request = RequestSpec {
            method: Method("GET".to_string()),
            url: "https://example.org".to_string(),
            ..Default::default()
        };
        let context_dir = ContextDir::default();
        let file = Output::File("/tmp/foo.bin".to_string());
        let output = Some(&file);
        let options = ClientOptions {
            aws_sigv4: Some("aws:amz:sts".to_string()),
            cacert_file: Some("/etc/cert.pem".to_string()),
            compressed: true,
            connects_to: vec!["example.com:443:host-47.example.com:443".to_string()],
            insecure: true,
            max_redirect: Some(10),
            path_as_is: true,
            proxy: Some("localhost:3128".to_string()),
            no_proxy: None,
            unix_socket: Some("/var/run/example.sock".to_string()),
            user: Some("user:password".to_string()),
            user_agent: Some("my-useragent".to_string()),
            verbosity: Some(Verbosity::VeryVerbose),
            ..Default::default()
        };

        let cmd = client.curl_command_line(&request, &context_dir, output, &options);
        assert_eq!(
            cmd,
            "curl \
         --aws-sigv4 aws:amz:sts \
         --cacert /etc/cert.pem \
         --compressed \
         --connect-to example.com:443:host-47.example.com:443 \
         --insecure \
         --max-redirs 10 \
         --path-as-is \
         --proxy 'localhost:3128' \
         --unix-socket '/var/run/example.sock' \
         --user 'user:password' \
         --user-agent 'my-useragent' \
         --output /tmp/foo.bin \
         'https://example.org'"
        );
    }

    #[test]
    fn parse_cert_option() {
        assert_eq!(parse_cert_password("foobar"), ("foobar".to_string(), None));
        assert_eq!(
            parse_cert_password("foobar:toto"),
            ("foobar".to_string(), Some("toto".to_string()))
        );
        assert_eq!(
            parse_cert_password("foobar:toto:tata"),
            ("foobar".to_string(), Some("toto:tata".to_string()))
        );
        assert_eq!(
            parse_cert_password("foobar:"),
            ("foobar".to_string(), Some("".to_string()))
        );
        assert_eq!(
            parse_cert_password("foobar\\"),
            ("foobar\\".to_string(), None)
        );
        assert_eq!(
            parse_cert_password("foo\\:bar\\:baz:toto:tata\\:tutu"),
            (
                "foo:bar:baz".to_string(),
                Some("toto:tata\\:tutu".to_string())
            )
        );
        assert_eq!(
            parse_cert_password("foo\\\\:toto\\:tata:tutu"),
            ("foo\\".to_string(), Some("toto\\:tata:tutu".to_string()))
        );
    }
}
