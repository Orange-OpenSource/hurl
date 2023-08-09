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
use clap::{value_parser, ArgAction};

pub fn aws_sigv4() -> clap::Arg {
    clap::Arg::new("aws_sigv4")
        .long("aws-sigv4")
        .value_name("PROVIDER1[:PROVIDER2[:REGION[:SERVICE]]]")
        .help("Use AWS V4 signature authentication in the transfer")
        .num_args(1)
}

pub fn cacert_file() -> clap::Arg {
    clap::Arg::new("cacert_file")
        .long("cacert")
        .value_name("FILE")
        .help("CA certificate to verify peer against (PEM format)")
        .num_args(1)
}

pub fn client_cert_file() -> clap::Arg {
    clap::Arg::new("client_cert_file")
        .short('E')
        .long("cert")
        .value_name("CERTIFICATE[:PASSWORD]")
        .help("Client certificate file and password")
        .num_args(1)
}

pub fn client_key_file() -> clap::Arg {
    clap::Arg::new("client_key_file")
        .long("key")
        .value_name("KEY")
        .help("Private key file name")
        .num_args(1)
}

pub fn color() -> clap::Arg {
    clap::Arg::new("color")
        .long("color")
        .help("Colorize output")
        .action(ArgAction::SetTrue)
        .conflicts_with("no_color")
}

pub fn compressed() -> clap::Arg {
    clap::Arg::new("compressed")
        .long("compressed")
        .help("Request compressed response (using deflate or gzip)")
        .action(ArgAction::SetTrue)
}

pub fn connect_timeout() -> clap::Arg {
    clap::Arg::new("connect_timeout")
        .long("connect-timeout")
        .value_name("SECONDS")
        .help("Maximum time allowed for connection")
        .default_value("300".to_string())
        .value_parser(value_parser!(u64))
        .num_args(1)
}

pub fn connect_to() -> clap::Arg {
    clap::Arg::new("connect_to")
        .long("connect-to")
        .value_name("HOST1:PORT1:HOST2:PORT2")
        .help("For a request to the given HOST1:PORT1 pair, connect to HOST2:PORT2 instead")
        .action(ArgAction::Append)
        .number_of_values(1)
        .num_args(1)
}

pub fn continue_on_error() -> clap::Arg {
    clap::Arg::new("continue_on_error")
        .long("continue-on-error")
        .help("Continue executing requests even if an error occurs")
        .action(ArgAction::SetTrue)
}

pub fn cookies_input_file() -> clap::Arg {
    clap::Arg::new("cookies_input_file")
        .short('b')
        .long("cookie")
        .value_name("FILE")
        .help("Read cookies from FILE")
        .num_args(1)
}

pub fn cookies_output_file() -> clap::Arg {
    clap::Arg::new("cookies_output_file")
        .short('c')
        .long("cookie-jar")
        .value_name("FILE")
        .help("Write cookies to FILE after running the session (only for one session)")
        .num_args(1)
}

pub fn delay() -> clap::Arg {
    clap::Arg::new("delay")
        .long("delay")
        .value_name("milliseconds")
        .help("Sets delay before each request.")
        .default_value("0")
        .value_parser(value_parser!(u64))
        .num_args(1)
}

pub fn error_format() -> clap::Arg {
    clap::Arg::new("error_format")
        .long("error-format")
        .value_name("FORMAT")
        .value_parser(["short", "long"])
        .default_value("short")
        .help("Control the format of error messages")
}

pub fn fail_at_end() -> clap::Arg {
    clap::Arg::new("fail_at_end")
        .long("fail-at-end")
        .help("Fail at end")
        .action(ArgAction::SetTrue)
}

pub fn file_root() -> clap::Arg {
    clap::Arg::new("file_root")
        .long("file-root")
        .value_name("DIR")
        .help("Set root filesystem to import files [default: current directory]")
        .num_args(1)
}

pub fn follow_location() -> clap::Arg {
    clap::Arg::new("follow_location")
        .short('L')
        .long("location")
        .help("Follow redirects")
        .action(ArgAction::SetTrue)
}

pub fn glob() -> clap::Arg {
    clap::Arg::new("glob")
        .long("glob")
        .value_name("GLOB")
        .help("Specify input files that match the given GLOB. Multiple glob flags may be used")
        .action(ArgAction::Append)
        .number_of_values(1)
}

pub fn http10() -> clap::Arg {
    clap::Arg::new("http10")
        .short('0')
        .long("http1.0")
        .help("Tells Hurl to use HTTP version 1.0 instead of using its internally preferred HTTP version")
        .action(ArgAction::SetTrue)
}

pub fn include() -> clap::Arg {
    clap::Arg::new("include")
        .short('i')
        .long("include")
        .help("Include the HTTP headers in the output")
        .action(ArgAction::SetTrue)
}

pub fn ignore_asserts() -> clap::Arg {
    clap::Arg::new("ignore_asserts")
        .long("ignore-asserts")
        .help("Ignore asserts defined in the Hurl file")
        .action(ArgAction::SetTrue)
}

pub fn input_files() -> clap::Arg {
    clap::Arg::new("input_files")
        .help("Sets the input file to use")
        .required(false)
        .index(1)
        .num_args(1..)
}

pub fn insecure() -> clap::Arg {
    clap::Arg::new("insecure")
        .short('k')
        .long("insecure")
        .help("Allow insecure SSL connections")
        .action(ArgAction::SetTrue)
}

pub fn interactive() -> clap::Arg {
    clap::Arg::new("interactive")
        .long("interactive")
        .help("Turn on interactive mode")
        .conflicts_with("to_entry")
        .action(ArgAction::SetTrue)
}

pub fn json() -> clap::Arg {
    clap::Arg::new("json")
        .long("json")
        .help("Output each Hurl file result to JSON")
        .conflicts_with("no_output")
        .action(ArgAction::SetTrue)
}

pub fn max_redirects() -> clap::Arg {
    clap::Arg::new("max_redirects")
        .long("max-redirs")
        .value_name("NUM")
        .help("Maximum number of redirects allowed, -1 for unlimited redirects")
        .default_value("50".to_string())
        .allow_hyphen_values(true)
        .value_parser(value_parser!(i32).range(-1..))
        .num_args(1)
}

pub fn max_time() -> clap::Arg {
    clap::Arg::new("max_time")
        .long("max-time")
        .short('m')
        .value_name("SECONDS")
        .help("Maximum time allowed for the transfer")
        .default_value("300".to_string())
        .allow_hyphen_values(true)
        .value_parser(value_parser!(u64))
        .num_args(1)
}

pub fn no_color() -> clap::Arg {
    clap::Arg::new("no_color")
        .long("no-color")
        .help("Do not colorize output")
        .conflicts_with("color")
        .action(ArgAction::SetTrue)
}

pub fn no_output() -> clap::Arg {
    clap::Arg::new("no_output")
        .long("no-output")
        .help("Suppress output. By default, Hurl outputs the body of the last response")
        .conflicts_with("json")
        .action(ArgAction::SetTrue)
}

pub fn noproxy() -> clap::Arg {
    clap::Arg::new("noproxy")
        .long("noproxy")
        .value_name("HOST(S)")
        .help("List of hosts which do not use proxy")
        .num_args(1)
}

pub fn output() -> clap::Arg {
    clap::Arg::new("output")
        .short('o')
        .long("output")
        .value_name("FILE")
        .help("Write to FILE instead of stdout")
        .num_args(1)
}

pub fn path_as_is() -> clap::Arg {
    clap::Arg::new("path_as_is")
        .long("path-as-is")
        .help("Tell Hurl to not handle sequences of /../ or /./ in the given URL path")
        .action(ArgAction::SetTrue)
}

pub fn proxy() -> clap::Arg {
    clap::Arg::new("proxy")
        .short('x')
        .long("proxy")
        .value_name("[PROTOCOL://]HOST[:PORT]")
        .help("Use proxy on given protocol/host/port")
        .num_args(1)
}

pub fn report_junit() -> clap::Arg {
    clap::Arg::new("junit")
        .long("report-junit")
        .value_name("FILE")
        .help("Write a JUnit XML report to FILE")
        .num_args(1)
}

pub fn report_tap() -> clap::Arg {
    clap::Arg::new("tap")
        .long("report-tap")
        .value_name("FILE")
        .help("Write a TAP report to FILE")
        .num_args(1)
}

pub fn report_html() -> clap::Arg {
    clap::Arg::new("report_html")
        .long("report-html")
        .value_name("DIR")
        .help("Generate HTML report to DIR")
        .num_args(1)
}

pub fn resolve() -> clap::Arg {
    clap::Arg::new("resolve")
        .long("resolve")
        .value_name("HOST:PORT:ADDR")
        .help("Provide a custom address for a specific host and port pair")
        .action(ArgAction::Append)
        .number_of_values(1)
        .num_args(1)
}

pub fn retry() -> clap::Arg {
    clap::Arg::new("retry")
        .long("retry")
        .value_name("NUM")
        .help("Maximum number of retries, 0 for no retries, -1 for unlimited retries")
        .allow_hyphen_values(true)
        .default_value("0")
        .value_parser(value_parser!(i32).range(-1..))
        .num_args(1)
}

pub fn retry_interval() -> clap::Arg {
    clap::Arg::new("retry_interval")
        .long("retry-interval")
        .value_name("MILLISECONDS")
        .help("Interval in milliseconds before a retry")
        .value_parser(value_parser!(u64))
        .default_value("1000")
        .num_args(1)
}

pub fn ssl_no_revoke() -> clap::Arg {
    clap::Arg::new("ssl_no_revoke")
        .long("ssl-no-revoke")
        .help("(Windows) This option tells Hurl to disable certificate revocation checks. WARNING: this option loosens the SSL security, and by using this flag you ask for exactly that.")
        .action(ArgAction::SetTrue)
}

pub fn test() -> clap::Arg {
    clap::Arg::new("test")
        .long("test")
        .help("Activate test mode")
        .action(ArgAction::SetTrue)
}

pub fn to_entry() -> clap::Arg {
    clap::Arg::new("to_entry")
        .long("to-entry")
        .value_name("ENTRY_NUMBER")
        .help("Execute Hurl file to ENTRY_NUMBER (starting at 1)")
        .conflicts_with("interactive")
        .allow_hyphen_values(true)
        .value_parser(value_parser!(u32).range(1..))
        .num_args(1)
}

pub fn user() -> clap::Arg {
    clap::Arg::new("user")
        .short('u')
        .long("user")
        .value_name("USER:PASSWORD")
        .help("Add basic Authentication header to each request")
        .num_args(1)
}

pub fn user_agent() -> clap::Arg {
    clap::Arg::new("user_agent")
        .short('A')
        .long("user-agent")
        .value_name("NAME")
        .help("Specify the User-Agent string to send to the HTTP server")
        .num_args(1)
}

pub fn variable() -> clap::Arg {
    clap::Arg::new("variable")
        .long("variable")
        .value_name("NAME=VALUE")
        .help("Define a variable")
        .action(ArgAction::Append)
        .number_of_values(1)
        .num_args(1)
}

pub fn variables_file() -> clap::Arg {
    clap::Arg::new("variables_file")
        .long("variables-file")
        .value_name("FILE")
        .help("Define a properties file in which you define your variables")
        .action(ArgAction::Append)
        .num_args(1)
}

pub fn verbose() -> clap::Arg {
    clap::Arg::new("verbose")
        .long("verbose")
        .help("Turn verbose")
        .action(ArgAction::SetTrue)
}

pub fn very_verbose() -> clap::Arg {
    clap::Arg::new("very_verbose")
        .long("very-verbose")
        .help("Turn on verbose output, including HTTP response and libcurl logs")
        .action(ArgAction::SetTrue)
}
