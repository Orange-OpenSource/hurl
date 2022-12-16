/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2022 Orange
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
use crate::cli;
use crate::cli::CliOptions;
use crate::http::ContextDir;
use crate::runner::Verbosity;
use hurl_core::ast::Entry;
use std::path::Path;
use std::time::Duration;

pub struct RunnerOptionsBuilder {
    cacert_file: Option<String>,
    client_cert_file: Option<String>,
    client_key_file: Option<String>,
    compressed: bool,
    connect_timeout: Duration,
    connects_to: Vec<String>,
    context_dir: ContextDir,
    cookie_input_file: Option<String>,
    fail_fast: bool,
    follow_location: bool,
    ignore_asserts: bool,
    insecure: bool,
    max_redirect: Option<usize>,
    no_proxy: Option<String>,
    post_entry: Option<fn() -> bool>,
    pre_entry: Option<fn(Entry) -> bool>,
    proxy: Option<String>,
    resolves: Vec<String>,
    retry: bool,
    retry_interval: Duration,
    retry_max_count: Option<usize>,
    timeout: Duration,
    to_entry: Option<usize>,
    user: Option<String>,
    user_agent: Option<String>,
    verbosity: Option<Verbosity>,
}

impl Default for RunnerOptionsBuilder {
    fn default() -> Self {
        RunnerOptionsBuilder {
            cacert_file: None,
            client_cert_file: None,
            client_key_file: None,
            compressed: false,
            connect_timeout: Duration::from_secs(300),
            connects_to: vec![],
            context_dir: ContextDir::default(),
            cookie_input_file: None,
            fail_fast: true,
            follow_location: false,
            ignore_asserts: false,
            insecure: false,
            max_redirect: Some(50),
            no_proxy: None,
            post_entry: None,
            pre_entry: None,
            proxy: None,
            resolves: vec![],
            retry: false,
            retry_interval: Duration::from_millis(1000),
            retry_max_count: Some(10),
            timeout: Duration::from_secs(300),
            to_entry: None,
            user: None,
            user_agent: None,
            verbosity: None,
        }
    }
}

impl RunnerOptionsBuilder {
    /// Returns a new Hurl runner options builder with a default values.
    pub fn new() -> Self {
        RunnerOptionsBuilder::default()
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

    /// Sets root file system to import files in Hurl.
    ///
    /// This is used for both files in multipart form data and request body.
    pub fn context_dir(&mut self, context_dir: &ContextDir) -> &mut Self {
        self.context_dir = context_dir.clone();
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
        self.fail_fast = fail_fast;
        self
    }

    /// Sets follow redirect.
    ///
    /// To limit the amount of redirects to follow use [`max_redirect`]
    pub fn follow_location(&mut self, follow_location: bool) -> &mut Self {
        self.follow_location = follow_location;
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

    /// Sets list of hosts which do not use a proxy.
    pub fn no_proxy(&mut self, no_proxy: Option<String>) -> &mut Self {
        self.no_proxy = no_proxy;
        self
    }

    /// Sets function to be executed after each entry excution.
    ///
    /// If the function returns true, the run is stopped.
    pub fn post_entry(&mut self, post_entry: Option<fn() -> bool>) -> &mut Self {
        self.post_entry = post_entry;
        self
    }

    /// Sets function to be executed before each entry excution.
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

    /// Retries requests if any error occurs (asserts, captures, runtimes etc...).
    ///
    /// Default is false.
    pub fn retry(&mut self, retry: bool) -> &mut Self {
        self.retry = retry;
        self
    }

    /// Sets suration between each retry.
    ///
    /// Default is 1000 ms.
    pub fn retry_interval(&mut self, retry_interval: Duration) -> &mut Self {
        self.retry_interval = retry_interval;
        self
    }

    /// Sets maximum number of retries.
    ///
    /// Default is 10.
    pub fn retry_max_count(&mut self, retry_max_count: Option<usize>) -> &mut Self {
        self.retry_max_count = retry_max_count;
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

    /// Sets verbosity level.
    pub fn verbosity(&mut self, verbosity: Option<Verbosity>) -> &mut Self {
        self.verbosity = verbosity;
        self
    }

    /// Create an instance of [`RunnerOptions`].
    pub fn build(&self) -> RunnerOptions {
        RunnerOptions {
            cacert_file: self.cacert_file.clone(),
            client_cert_file: self.client_cert_file.clone(),
            client_key_file: self.client_key_file.clone(),
            compressed: self.compressed,
            connect_timeout: self.connect_timeout,
            connects_to: self.connects_to.clone(),
            context_dir: self.context_dir.clone(),
            cookie_input_file: self.cookie_input_file.clone(),
            fail_fast: self.fail_fast,
            follow_location: self.follow_location,
            ignore_asserts: self.ignore_asserts,
            insecure: self.insecure,
            max_redirect: self.max_redirect,
            no_proxy: self.no_proxy.clone(),
            post_entry: self.post_entry,
            pre_entry: self.pre_entry,
            proxy: self.proxy.clone(),
            resolves: self.resolves.clone(),
            retry: self.retry,
            retry_interval: self.retry_interval,
            retry_max_count: self.retry_max_count,
            timeout: self.timeout,
            to_entry: self.to_entry,
            user: self.user.clone(),
            user_agent: self.user_agent.clone(),
            verbosity: self.verbosity.clone(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RunnerOptions {
    pub(crate) cacert_file: Option<String>,
    pub(crate) client_cert_file: Option<String>,
    pub(crate) client_key_file: Option<String>,
    pub(crate) compressed: bool,
    pub(crate) connect_timeout: Duration,
    pub(crate) connects_to: Vec<String>,
    pub(crate) context_dir: ContextDir,
    pub(crate) cookie_input_file: Option<String>,
    pub(crate) fail_fast: bool,
    pub(crate) follow_location: bool,
    pub(crate) ignore_asserts: bool,
    pub(crate) insecure: bool,
    pub(crate) max_redirect: Option<usize>,
    pub(crate) no_proxy: Option<String>,
    pub(crate) post_entry: Option<fn() -> bool>,
    pub(crate) pre_entry: Option<fn(Entry) -> bool>,
    pub(crate) proxy: Option<String>,
    pub(crate) resolves: Vec<String>,
    pub(crate) retry: bool,
    pub(crate) retry_interval: Duration,
    pub(crate) retry_max_count: Option<usize>,
    pub(crate) timeout: Duration,
    pub(crate) to_entry: Option<usize>,
    pub(crate) user: Option<String>,
    pub(crate) user_agent: Option<String>,
    pub(crate) verbosity: Option<Verbosity>,
}

impl Default for RunnerOptions {
    fn default() -> Self {
        RunnerOptionsBuilder::default().build()
    }
}

impl RunnerOptions {
    pub fn from(filename: &str, current_dir: &Path, cli_options: &CliOptions) -> Self {
        let cacert_file = cli_options.cacert_file.clone();
        let client_cert_file = cli_options.client_cert_file.clone();
        let client_key_file = cli_options.client_key_file.clone();
        let connects_to = cli_options.connects_to.clone();
        let follow_location = cli_options.follow_location;
        let verbosity = match (cli_options.verbose, cli_options.very_verbose) {
            (true, true) => Some(Verbosity::VeryVerbose),
            (true, _) => Some(Verbosity::Verbose),
            _ => None,
        };
        let insecure = cli_options.insecure;
        let max_redirect = cli_options.max_redirect;
        let proxy = cli_options.proxy.clone();
        let no_proxy = cli_options.no_proxy.clone();
        let cookie_input_file = cli_options.cookie_input_file.clone();
        let timeout = cli_options.timeout;
        let connect_timeout = cli_options.connect_timeout;
        let user = cli_options.user.clone();
        let user_agent = cli_options.user_agent.clone();
        let compressed = cli_options.compressed;
        let file_root = match cli_options.file_root {
            Some(ref filename) => Path::new(filename),
            None => {
                if filename == "-" {
                    current_dir
                } else {
                    let path = Path::new(filename);
                    path.parent().unwrap()
                }
            }
        };
        let context_dir = ContextDir::new(current_dir, file_root);
        let pre_entry = if cli_options.interactive {
            Some(cli::interactive::pre_entry as fn(Entry) -> bool)
        } else {
            None
        };
        let post_entry = if cli_options.interactive {
            Some(cli::interactive::post_entry as fn() -> bool)
        } else {
            None
        };
        let fail_fast = cli_options.fail_fast;
        let to_entry = cli_options.to_entry;
        let resolves = cli_options.resolves.clone();
        let retry = cli_options.retry;
        let retry_interval = cli_options.retry_interval;
        let retry_max_count = cli_options.retry_max_count;
        let ignore_asserts = cli_options.ignore_asserts;

        RunnerOptions {
            cacert_file,
            client_cert_file,
            client_key_file,
            compressed,
            connect_timeout,
            connects_to,
            context_dir,
            cookie_input_file,
            fail_fast,
            follow_location,
            ignore_asserts,
            insecure,
            max_redirect,
            no_proxy,
            post_entry,
            pre_entry,
            proxy,
            resolves,
            retry,
            retry_interval,
            retry_max_count,
            timeout,
            to_entry,
            user,
            user_agent,
            verbosity,
        }
    }
}
