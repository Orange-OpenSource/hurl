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
use std::time::Duration;

use hurl_core::ast::Entry;
use hurl_core::typing::{BytesPerSec, Count};

use crate::http::{IpResolve, RequestedHttpVersion};
use crate::runner::Output;
use crate::util::path::ContextDir;

/// Build a [`RunnerOptions`] instance.
pub struct RunnerOptionsBuilder {
    allow_reuse: bool,
    aws_sigv4: Option<String>,
    cacert_file: Option<String>,
    client_cert_file: Option<String>,
    client_key_file: Option<String>,
    compressed: bool,
    connect_timeout: Duration,
    connects_to: Vec<String>,
    context_dir: ContextDir,
    continue_on_error: bool,
    cookie_input_file: Option<String>,
    delay: Duration,
    follow_location: bool,
    follow_location_trusted: bool,
    from_entry: Option<usize>,
    headers: Vec<String>,
    http_version: RequestedHttpVersion,
    ignore_asserts: bool,
    insecure: bool,
    ip_resolve: IpResolve,
    max_filesize: Option<u64>,
    max_recv_speed: Option<BytesPerSec>,
    max_redirect: Count,
    max_send_speed: Option<BytesPerSec>,
    netrc: bool,
    netrc_file: Option<String>,
    netrc_optional: bool,
    no_proxy: Option<String>,
    output: Option<Output>,
    path_as_is: bool,
    post_entry: Option<fn() -> bool>,
    pre_entry: Option<fn(&Entry) -> bool>,
    proxy: Option<String>,
    repeat: Option<Count>,
    resolves: Vec<String>,
    retry: Option<Count>,
    retry_interval: Duration,
    skip: bool,
    ssl_no_revoke: bool,
    timeout: Duration,
    to_entry: Option<usize>,
    unix_socket: Option<String>,
    user: Option<String>,
    user_agent: Option<String>,
}

impl Default for RunnerOptionsBuilder {
    fn default() -> Self {
        RunnerOptionsBuilder {
            allow_reuse: true,
            aws_sigv4: None,
            cacert_file: None,
            client_cert_file: None,
            client_key_file: None,
            compressed: false,
            connect_timeout: Duration::from_secs(300),
            connects_to: vec![],
            context_dir: ContextDir::default(),
            continue_on_error: false,
            cookie_input_file: None,
            delay: Duration::from_millis(0),
            follow_location: false,
            follow_location_trusted: false,
            from_entry: None,
            headers: vec![],
            http_version: RequestedHttpVersion::default(),
            ignore_asserts: false,
            insecure: false,
            ip_resolve: IpResolve::default(),
            max_filesize: None,
            max_recv_speed: None,
            max_redirect: Count::Finite(50),
            max_send_speed: None,
            netrc: false,
            netrc_file: None,
            netrc_optional: false,
            no_proxy: None,
            output: None,
            path_as_is: false,
            post_entry: None,
            pre_entry: None,
            proxy: None,
            repeat: None,
            resolves: vec![],
            retry: None,
            retry_interval: Duration::from_millis(1000),
            skip: false,
            ssl_no_revoke: false,
            timeout: Duration::from_secs(300),
            to_entry: None,
            unix_socket: None,
            user: None,
            user_agent: None,
        }
    }
}

impl RunnerOptionsBuilder {
    /// Returns a new Hurl runner options builder with a default values.
    pub fn new() -> Self {
        RunnerOptionsBuilder::default()
    }

    /// Allow reusing internal connections, `true` by default. Setting this to `false` forces the
    /// HTTP client to use a new HTTP connection, and also marks this new connection as not reusable.
    /// The main use-case for not allowing connection reuse is when we want to switch HTTP version
    /// mid-file with an `[Options]` section. As the HTTP version setter is just a query, and is not
    /// always honored by libcurl when reusing connection, this allows to be sure that the client
    /// will set the queried HTTP version.
    pub fn allow_reuse(&mut self, allow_reuse: bool) -> &mut Self {
        self.allow_reuse = allow_reuse;
        self
    }

    /// Specifies the AWS SigV4 option
    pub fn aws_sigv4(&mut self, aws_sigv4: Option<String>) -> &mut Self {
        self.aws_sigv4 = aws_sigv4;
        self
    }

    /// Specifies the certificate file for peer verification.
    /// The file may contain multiple CA certificates and must be in PEM format.
    pub fn cacert_file(&mut self, cacert_file: Option<String>) -> &mut Self {
        self.cacert_file = cacert_file;
        self
    }

    /// Sets Client certificate file and password.
    pub fn client_cert_file(&mut self, client_cert_file: Option<String>) -> &mut Self {
        self.client_cert_file = client_cert_file;
        self
    }

    /// Sets private key file name.
    pub fn client_key_file(&mut self, client_key_file: Option<String>) -> &mut Self {
        self.client_key_file = client_key_file;
        self
    }

    /// Requests a compressed response using one of the algorithms br, gzip, deflate and
    /// automatically decompress the content.
    pub fn compressed(&mut self, compressed: bool) -> &mut Self {
        self.compressed = compressed;
        self
    }

    /// Sets maximum time that you allow Hurl’s connection to take.
    ///
    /// Default 300 seconds.
    pub fn connect_timeout(&mut self, connect_timeout: Duration) -> &mut Self {
        self.connect_timeout = connect_timeout;
        self
    }

    /// Sets hosts mappings.
    ///
    /// Each value has the following format HOST1:PORT1:HOST2:PORT2
    /// For a request to the given HOST1:PORT1 pair, connect to HOST2:PORT2 instead.
    pub fn connects_to(&mut self, connects_to: &[String]) -> &mut Self {
        self.connects_to = connects_to.to_vec();
        self
    }

    /// Sets delay (timeout) before the request.
    ///
    /// Default is 0 ms.
    pub fn delay(&mut self, delay: Duration) -> &mut Self {
        self.delay = delay;
        self
    }

    /// Sets root file system to import files in Hurl.
    ///
    /// This is used for both files in multipart form data and request body.
    pub fn context_dir(&mut self, context_dir: &ContextDir) -> &mut Self {
        self.context_dir = context_dir.clone();
        self
    }

    /// Sets stopping or continuing executing requests to the end of the Hurl file even when an error occurs.
    ///
    /// By default, Hurl exits after an error in the HTTP response. Note that this option does
    /// not affect the behavior with multiple input Hurl files.
    pub fn continue_on_error(&mut self, continue_on_error: bool) -> &mut Self {
        self.continue_on_error = continue_on_error;
        self
    }

    /// Reads cookies from this file (using the Netscape cookie file format).
    pub fn cookie_input_file(&mut self, cookie_input_file: Option<String>) -> &mut Self {
        self.cookie_input_file = cookie_input_file;
        self
    }

    /// Sets stopping or continuing executing requests to the end of the Hurl file even when an assert error occurs.
    ///
    /// By default, Hurl exits after an assert error in the HTTP response. Note that this option does
    /// not affect the behavior with multiple input Hurl files.
    pub fn fail_fast(&mut self, fail_fast: bool) -> &mut Self {
        self.continue_on_error = !fail_fast;
        self
    }

    /// Sets follow redirect.
    ///
    /// To limit the amount of redirects to follow use [`self.max_redirect()`]
    pub fn follow_location(&mut self, follow_location: bool) -> &mut Self {
        self.follow_location = follow_location;
        self
    }

    /// Sets follow redirect with trust.
    ///
    /// To limit the amount of redirects to follow use [`self.max_redirect()`]
    pub fn follow_location_trusted(&mut self, follow_location_trusted: bool) -> &mut Self {
        self.follow_location_trusted = follow_location_trusted;
        self
    }

    /// Executes Hurl file from `from_entry` (starting at 1), ignores the beginning of the file.
    pub fn from_entry(&mut self, from_entry: Option<usize>) -> &mut Self {
        self.from_entry = from_entry;
        self
    }

    /// Sets additional headers (overrides if a header already exists).
    pub fn headers(&mut self, header: &[String]) -> &mut Self {
        self.headers = header.to_vec();
        self
    }

    /// Set requested HTTP version (can be different of the effective HTTP version).
    pub fn http_version(&mut self, version: RequestedHttpVersion) -> &mut Self {
        self.http_version = version;
        self
    }

    /// Ignores all asserts defined in the Hurl file.
    pub fn ignore_asserts(&mut self, ignore_asserts: bool) -> &mut Self {
        self.ignore_asserts = ignore_asserts;
        self
    }

    /// Allows Hurl to perform “insecure” SSL connections and transfers.
    pub fn insecure(&mut self, insecure: bool) -> &mut Self {
        self.insecure = insecure;
        self
    }

    /// Set IP version.
    pub fn ip_resolve(&mut self, ip_resolve: IpResolve) -> &mut Self {
        self.ip_resolve = ip_resolve;
        self
    }

    /// Set the file size limit
    pub fn max_filesize(&mut self, max_filesize: Option<u64>) -> &mut Self {
        self.max_filesize = max_filesize;
        self
    }

    /// Set maximum number of redirection-followings allowed
    ///
    /// By default, the limit is set to 50 redirections
    pub fn max_redirect(&mut self, max_redirect: Count) -> &mut Self {
        self.max_redirect = max_redirect;
        self
    }

    /// Set the maximum upload speed.
    pub fn max_send_speed(&mut self, max_send_speed: Option<BytesPerSec>) -> &mut Self {
        self.max_send_speed = max_send_speed;
        self
    }

    /// Set the maximum download speed.
    pub fn max_recv_speed(&mut self, max_recv_speed: Option<BytesPerSec>) -> &mut Self {
        self.max_recv_speed = max_recv_speed;
        self
    }

    /// Sets the path-as-is flag.
    pub fn path_as_is(&mut self, path_as_is: bool) -> &mut Self {
        self.path_as_is = path_as_is;
        self
    }

    /// Sets the netrc flag.
    pub fn netrc(&mut self, netrc: bool) -> &mut Self {
        self.netrc = netrc;
        self
    }

    /// Sets the netrc file.
    pub fn netrc_file(&mut self, netrc_file: Option<String>) -> &mut Self {
        self.netrc_file = netrc_file;
        self
    }

    /// Sets the optional netrc flag.
    pub fn netrc_optional(&mut self, netrc_optional: bool) -> &mut Self {
        self.netrc_optional = netrc_optional;
        self
    }

    /// Sets list of hosts which do not use a proxy.
    pub fn no_proxy(&mut self, no_proxy: Option<String>) -> &mut Self {
        self.no_proxy = no_proxy;
        self
    }

    /// Specifies the file to output the HTTP response instead of stdout.
    pub fn output(&mut self, output: Option<Output>) -> &mut Self {
        self.output = output;
        self
    }

    /// Sets function to be executed after each entry execution.
    ///
    /// If the function returns true, the run is stopped.
    pub fn post_entry(&mut self, post_entry: Option<fn() -> bool>) -> &mut Self {
        self.post_entry = post_entry;
        self
    }

    /// Sets function to be executed before each entry execution.
    ///
    /// If the function returns true, the run is stopped.
    pub fn pre_entry(&mut self, pre_entry: Option<fn(&Entry) -> bool>) -> &mut Self {
        self.pre_entry = pre_entry;
        self
    }

    /// Sets the specified proxy to be used.
    pub fn proxy(&mut self, proxy: Option<String>) -> &mut Self {
        self.proxy = proxy;
        self
    }

    /// Set the number of repetition for a given entry.
    pub fn repeat(&mut self, repeat: Option<Count>) -> &mut Self {
        self.repeat = repeat;
        self
    }

    /// Provides a custom address for a specific host and port pair.
    pub fn resolves(&mut self, resolves: &[String]) -> &mut Self {
        self.resolves = resolves.to_vec();
        self
    }

    /// Sets maximum number of retries.
    ///
    /// Default is 0.
    pub fn retry(&mut self, retry: Option<Count>) -> &mut Self {
        self.retry = retry;
        self
    }

    /// Sets duration between each retry.
    ///
    /// Default is 1000 ms.
    pub fn retry_interval(&mut self, retry_interval: Duration) -> &mut Self {
        self.retry_interval = retry_interval;
        self
    }

    /// Skip the run without executing any request.
    pub fn skip(&mut self, skip: bool) -> &mut Self {
        self.skip = skip;
        self
    }

    /// Disables certificate revocation checks for SSL backends where such behavior is present.
    pub fn ssl_no_revoke(&mut self, ssl_no_revoke: bool) -> &mut Self {
        self.ssl_no_revoke = ssl_no_revoke;
        self
    }

    /// Sets maximum time allowed for the transfer.
    ///
    /// Default 300 seconds.
    pub fn timeout(&mut self, timeout: Duration) -> &mut Self {
        self.timeout = timeout;
        self
    }

    /// Executes Hurl file to `to_entry` (starting at 1), ignores the remaining of the file.
    pub fn to_entry(&mut self, to_entry: Option<usize>) -> &mut Self {
        self.to_entry = to_entry;
        self
    }

    /// Sets the specified unix domain socket to connect through, instead of using the network.
    pub fn unix_socket(&mut self, unix_socket: Option<String>) -> &mut Self {
        self.unix_socket = unix_socket;
        self
    }

    /// Adds basic Authentication header to each request.
    pub fn user(&mut self, user: Option<String>) -> &mut Self {
        self.user = user;
        self
    }

    /// Specifies the User-Agent string to send to the HTTP server.
    pub fn user_agent(&mut self, user_agent: Option<String>) -> &mut Self {
        self.user_agent = user_agent;
        self
    }

    /// Create an instance of [`RunnerOptions`].
    pub fn build(&self) -> RunnerOptions {
        RunnerOptions {
            allow_reuse: self.allow_reuse,
            aws_sigv4: self.aws_sigv4.clone(),
            cacert_file: self.cacert_file.clone(),
            client_cert_file: self.client_cert_file.clone(),
            client_key_file: self.client_key_file.clone(),
            compressed: self.compressed,
            connect_timeout: self.connect_timeout,
            connects_to: self.connects_to.clone(),
            delay: self.delay,
            context_dir: self.context_dir.clone(),
            continue_on_error: self.continue_on_error,
            cookie_input_file: self.cookie_input_file.clone(),
            follow_location: self.follow_location,
            follow_location_trusted: self.follow_location_trusted,
            from_entry: self.from_entry,
            headers: self.headers.clone(),
            http_version: self.http_version,
            ignore_asserts: self.ignore_asserts,
            insecure: self.insecure,
            ip_resolve: self.ip_resolve,
            max_filesize: self.max_filesize,
            max_recv_speed: self.max_recv_speed,
            max_redirect: self.max_redirect,
            max_send_speed: self.max_send_speed,
            netrc: self.netrc,
            netrc_file: self.netrc_file.clone(),
            netrc_optional: self.netrc_optional,
            no_proxy: self.no_proxy.clone(),
            output: self.output.clone(),
            path_as_is: self.path_as_is,
            post_entry: self.post_entry,
            pre_entry: self.pre_entry,
            proxy: self.proxy.clone(),
            repeat: self.repeat,
            resolves: self.resolves.clone(),
            retry: self.retry,
            retry_interval: self.retry_interval,
            skip: self.skip,
            ssl_no_revoke: self.ssl_no_revoke,
            timeout: self.timeout,
            to_entry: self.to_entry,
            unix_socket: self.unix_socket.clone(),
            user: self.user.clone(),
            user_agent: self.user_agent.clone(),
        }
    }
}

/// Represents the configuration options to run an Hurl file.
///
/// Most options are used to configure the HTTP client used for running requests, while other
/// are used to configure asserts settings, output etc....
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RunnerOptions {
    /// Allow reusing internal connections.
    pub(crate) allow_reuse: bool,
    /// Specifies the AWS SigV4 option.
    pub(crate) aws_sigv4: Option<String>,
    /// Specifies the certificate file for peer verification.
    pub(crate) cacert_file: Option<String>,
    /// Sets Client certificate file and password.
    pub(crate) client_cert_file: Option<String>,
    /// Sets private key file name.
    pub(crate) client_key_file: Option<String>,
    /// Requests a compressed response using one of the algorithms br, gzip, deflate and
    /// automatically decompress the content.
    pub(crate) compressed: bool,
    /// Sets maximum time that you allow Hurl’s connection to take.
    pub(crate) connect_timeout: Duration,
    /// Sets hosts mappings.
    pub(crate) connects_to: Vec<String>,
    /// Sets delay (timeout) before the request.
    pub(crate) delay: Duration,
    /// Sets root file system to import files in Hurl.
    pub(crate) context_dir: ContextDir,
    /// Sets stopping or continuing executing requests to the end of the Hurl file even when an error occurs.
    pub(crate) continue_on_error: bool,
    /// Reads cookies from this file (using the Netscape cookie file format).
    pub(crate) cookie_input_file: Option<String>,
    /// Sets follow redirect.
    pub(crate) follow_location: bool,
    /// Sets follow redirect with trust.
    pub(crate) follow_location_trusted: bool,
    /// Executes Hurl file from from_entry (starting at 1), ignores the beginning of the file.
    pub(crate) from_entry: Option<usize>,
    /// Sets additional headers (overrides if a header already exists).
    pub(crate) headers: Vec<String>,
    /// Set requested HTTP version (can be different of the effective HTTP version).
    pub(crate) http_version: RequestedHttpVersion,
    /// Ignores all asserts defined in the Hurl file.
    pub(crate) ignore_asserts: bool,
    /// Set IP version.
    pub(crate) ip_resolve: IpResolve,
    /// Allows Hurl to perform “insecure” SSL connections and transfers.
    pub(crate) insecure: bool,
    /// Set the file size limit.
    pub(crate) max_filesize: Option<u64>,
    /// Set the maximum download speed.
    pub(crate) max_recv_speed: Option<BytesPerSec>,
    /// Set maximum number of redirection-followings allowed.
    pub(crate) max_redirect: Count,
    /// Set the maximum upload speed.
    pub(crate) max_send_speed: Option<BytesPerSec>,
    /// Sets the netrc flag.
    pub(crate) netrc: bool,
    /// Sets the netrc file.
    pub(crate) netrc_file: Option<String>,
    /// Sets the optional netrc flag.
    pub(crate) netrc_optional: bool,
    /// Sets list of hosts which do not use a proxy.
    pub(crate) no_proxy: Option<String>,
    /// Specifies the file to output the HTTP response.
    pub(crate) output: Option<Output>,
    pub(crate) path_as_is: bool,
    /// Sets function to be executed before each entry execution.
    pub(crate) post_entry: Option<fn() -> bool>,
    /// Sets function to be executed after each entry execution.
    pub(crate) pre_entry: Option<fn(&Entry) -> bool>,
    /// Sets the specified proxy to be used.
    pub(crate) proxy: Option<String>,
    /// Set the number of repetition for a given entry.
    pub(crate) repeat: Option<Count>,
    /// Provides a custom address for a specific host and port pair.
    pub(crate) resolves: Vec<String>,
    /// Sets maximum number of retries.
    pub(crate) retry: Option<Count>,
    /// Sets duration between each retry.
    pub(crate) retry_interval: Duration,
    /// Skip the run without executing any request.
    pub(crate) skip: bool,
    /// Disables certificate revocation checks for SSL backends where such behavior is present.
    pub(crate) ssl_no_revoke: bool,
    /// Sets maximum time allowed for the transfer.
    pub(crate) timeout: Duration,
    /// Executes Hurl file to to_entry (starting at 1), ignores the remaining of the file.
    pub(crate) to_entry: Option<usize>,
    /// Sets the specified unix domain socket to connect through, instead of using the network.
    pub(crate) unix_socket: Option<String>,
    /// Adds basic Authentication header to each request.
    pub(crate) user: Option<String>,
    /// Specifies the User-Agent string to send to the HTTP server.
    pub(crate) user_agent: Option<String>,
}

impl Default for RunnerOptions {
    fn default() -> Self {
        RunnerOptionsBuilder::default().build()
    }
}
