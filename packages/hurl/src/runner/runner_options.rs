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
use std::time::Duration;

use crate::http::HttpVersion;
use hurl_core::ast::{Entry, Retry};

use crate::util::path::ContextDir;

pub struct RunnerOptionsBuilder {
    aws_sigv4: Option<String>,
    cacert_file: Option<String>,
    client_cert_file: Option<String>,
    client_key_file: Option<String>,
    compressed: bool,
    connect_timeout: Duration,
    connects_to: Vec<String>,
    delay: Duration,
    context_dir: ContextDir,
    continue_on_error: bool,
    cookie_input_file: Option<String>,
    follow_location: bool,
    http_version: Option<HttpVersion>,
    ignore_asserts: bool,
    insecure: bool,
    max_redirect: Option<usize>,
    no_proxy: Option<String>,
    path_as_is: bool,
    post_entry: Option<fn() -> bool>,
    pre_entry: Option<fn(Entry) -> bool>,
    proxy: Option<String>,
    resolves: Vec<String>,
    retry: Retry,
    retry_interval: Duration,
    ssl_no_revoke: bool,
    timeout: Duration,
    to_entry: Option<usize>,
    user: Option<String>,
    user_agent: Option<String>,
}

impl Default for RunnerOptionsBuilder {
    fn default() -> Self {
        RunnerOptionsBuilder {
            aws_sigv4: None,
            cacert_file: None,
            client_cert_file: None,
            client_key_file: None,
            compressed: false,
            connect_timeout: Duration::from_secs(300),
            connects_to: vec![],
            delay: Duration::from_millis(0),
            context_dir: ContextDir::default(),
            continue_on_error: false,
            cookie_input_file: None,
            follow_location: false,
            http_version: None,
            ignore_asserts: false,
            insecure: false,
            max_redirect: Some(50),
            no_proxy: None,
            path_as_is: false,
            post_entry: None,
            pre_entry: None,
            proxy: None,
            resolves: vec![],
            retry: Retry::None,
            retry_interval: Duration::from_millis(1000),
            ssl_no_revoke: false,
            timeout: Duration::from_secs(300),
            to_entry: None,
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

    pub fn http_version(&mut self, version: Option<HttpVersion>) -> &mut Self {
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

    /// Set maximum number of redirection-followings allowed
    ///
    /// By default, the limit is set to 50 redirections
    pub fn max_redirect(&mut self, max_redirect: Option<usize>) -> &mut Self {
        self.max_redirect = max_redirect;
        self
    }

    /// Sets the path-as-is flag.
    pub fn path_as_is(&mut self, path_as_is: bool) -> &mut Self {
        self.path_as_is = path_as_is;
        self
    }

    /// Sets list of hosts which do not use a proxy.
    pub fn no_proxy(&mut self, no_proxy: Option<String>) -> &mut Self {
        self.no_proxy = no_proxy;
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
    pub fn pre_entry(&mut self, pre_entry: Option<fn(Entry) -> bool>) -> &mut Self {
        self.pre_entry = pre_entry;
        self
    }

    /// Sets the specified proxy to be used.
    pub fn proxy(&mut self, proxy: Option<String>) -> &mut Self {
        self.proxy = proxy;
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
    pub fn retry(&mut self, retry: Retry) -> &mut Self {
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
            http_version: self.http_version,
            ignore_asserts: self.ignore_asserts,
            insecure: self.insecure,
            max_redirect: self.max_redirect,
            no_proxy: self.no_proxy.clone(),
            path_as_is: self.path_as_is,
            post_entry: self.post_entry,
            pre_entry: self.pre_entry,
            proxy: self.proxy.clone(),
            resolves: self.resolves.clone(),
            retry: self.retry,
            retry_interval: self.retry_interval,
            ssl_no_revoke: self.ssl_no_revoke,
            timeout: self.timeout,
            to_entry: self.to_entry,
            user: self.user.clone(),
            user_agent: self.user_agent.clone(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RunnerOptions {
    pub(crate) aws_sigv4: Option<String>,
    pub(crate) cacert_file: Option<String>,
    pub(crate) client_cert_file: Option<String>,
    pub(crate) client_key_file: Option<String>,
    pub(crate) compressed: bool,
    pub(crate) connect_timeout: Duration,
    pub(crate) connects_to: Vec<String>,
    pub(crate) delay: Duration,
    pub(crate) context_dir: ContextDir,
    pub(crate) continue_on_error: bool,
    pub(crate) cookie_input_file: Option<String>,
    pub(crate) follow_location: bool,
    pub(crate) http_version: Option<HttpVersion>,
    pub(crate) ignore_asserts: bool,
    pub(crate) insecure: bool,
    pub(crate) max_redirect: Option<usize>,
    pub(crate) no_proxy: Option<String>,
    pub(crate) path_as_is: bool,
    pub(crate) post_entry: Option<fn() -> bool>,
    pub(crate) pre_entry: Option<fn(Entry) -> bool>,
    pub(crate) proxy: Option<String>,
    pub(crate) resolves: Vec<String>,
    pub(crate) retry: Retry,
    pub(crate) retry_interval: Duration,
    pub(crate) ssl_no_revoke: bool,
    pub(crate) timeout: Duration,
    pub(crate) to_entry: Option<usize>,
    pub(crate) user: Option<String>,
    pub(crate) user_agent: Option<String>,
}

impl Default for RunnerOptions {
    fn default() -> Self {
        RunnerOptionsBuilder::default().build()
    }
}
