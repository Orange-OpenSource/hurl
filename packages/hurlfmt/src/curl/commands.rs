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
use clap::{value_parser, ArgAction};

pub fn compressed() -> clap::Arg {
    clap::Arg::new("compressed").long("compressed").num_args(0)
}

pub fn cookies() -> clap::Arg {
    clap::Arg::new("cookies")
        .long("cookie")
        .short('b')
        .value_name("NAME1=VALUE1; NAME2=VALUE2")
        .value_parser(|value: &str| {
            if value.trim().is_empty() {
                return Err("empty value provided".to_string());
            }

            let (valid_cookies, invalid_cookies): (Vec<_>, Vec<_>) = value
                .split(';')
                .map(str::trim)
                .filter(|c| !c.is_empty())
                .partition(|c| c.contains('='));

            if invalid_cookies.is_empty() {
                Ok(valid_cookies.join("; "))
            } else {
                match invalid_cookies.as_slice() {
                    [_] => Err("invalid cookie pair provided".to_string()),
                    _ => Err(format!(
                        "invalid cookie pairs provided: [{}]",
                        invalid_cookies.join(", ")
                    )),
                }
            }
        })
        .action(ArgAction::Append)
        .num_args(1)
}

pub fn data() -> clap::Arg {
    clap::Arg::new("data")
        .long("data")
        .short('d')
        .value_name("data")
        .num_args(1)
}

pub fn headers() -> clap::Arg {
    clap::Arg::new("headers")
        .long("header")
        .short('H')
        .value_name("NAME:VALUE")
        .value_parser(|value: &str| {
            // We add a basic format check on headers, accepting either "NAME: VALUE" or "NAME;" for an empty header.
            // See curl manual <https://curl.se/docs/manpage.html#-H>
            // > If you send the custom header with no-value then its header must be terminated with a semicolon,
            // > such as -H "X-Custom-Header;" to send "X-Custom-Header:".
            if value.contains(":") || value.ends_with(";") {
                Ok(String::from(value))
            } else {
                Err("headers must be formatted as '<NAME:VALUE>' or '<NAME>;'")
            }
        })
        .action(ArgAction::Append)
        .num_args(1)
}

pub fn insecure() -> clap::Arg {
    clap::Arg::new("insecure")
        .long("insecure")
        .short('k')
        .num_args(0)
}

pub fn location() -> clap::Arg {
    clap::Arg::new("location")
        .long("location")
        .short('L')
        .num_args(0)
}

pub fn max_redirects() -> clap::Arg {
    clap::Arg::new("max_redirects")
        .long("max-redirs")
        .value_name("NUM")
        .allow_hyphen_values(true)
        .value_parser(value_parser!(i32).range(-1..))
        .num_args(1)
}

pub fn method() -> clap::Arg {
    clap::Arg::new("method")
        .long("request")
        .short('X')
        .value_name("METHOD")
        .num_args(1)
}

pub fn retry() -> clap::Arg {
    clap::Arg::new("retry")
        .long("retry")
        .value_name("seconds")
        .value_parser(value_parser!(i32))
        .num_args(1)
}

pub fn url() -> clap::Arg {
    clap::Arg::new("url")
        .long("url")
        .value_name("url")
        .num_args(1)
}

pub fn url_param() -> clap::Arg {
    clap::Arg::new("url_param")
        .help("Sets the url to use")
        .required(false)
        .num_args(1)
}

pub fn verbose() -> clap::Arg {
    clap::Arg::new("verbose")
        .long("verbose")
        .short('v')
        .num_args(0)
}
