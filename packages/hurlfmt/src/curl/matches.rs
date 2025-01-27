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
use clap::ArgMatches;

use super::HurlOption;

pub fn body(arg_matches: &ArgMatches) -> Option<String> {
    match get_string(arg_matches, "data") {
        None => None,
        Some(v) => {
            if let Some(filename) = v.strip_prefix('@') {
                Some(format!("file, {filename};"))
            } else {
                Some(format!("```\n{v}\n```"))
            }
        }
    }
}

pub fn method(arg_matches: &ArgMatches) -> String {
    match get_string(arg_matches, "method") {
        None => {
            if arg_matches.contains_id("data") {
                "POST".to_string()
            } else {
                "GET".to_string()
            }
        }
        Some(v) => v,
    }
}

pub fn url(arg_matches: &ArgMatches) -> String {
    let s = if let Some(value) = get_string(arg_matches, "url") {
        value
    } else {
        get_string(arg_matches, "url_param").unwrap()
    };
    if !s.starts_with("http") {
        format!("https://{s}")
    } else {
        s
    }
}

pub fn headers(arg_matches: &ArgMatches) -> Vec<String> {
    let mut headers = get_strings(arg_matches, "headers").unwrap_or_default();
    if !has_content_type(&headers) {
        if let Some(data) = get_string(arg_matches, "data") {
            if !data.starts_with('@') {
                headers.push("Content-Type: application/x-www-form-urlencoded".to_string());
            }
        }
    }

    headers
}

pub fn options(arg_matches: &ArgMatches) -> Vec<HurlOption> {
    let mut options = vec![];
    if has_flag(arg_matches, "compressed") {
        options.push(HurlOption::new("compressed", "true"));
    }
    if has_flag(arg_matches, "location") {
        options.push(HurlOption::new("location", "true"));
    }
    if has_flag(arg_matches, "insecure") {
        options.push(HurlOption::new("insecure", "true"));
    }
    if let Some(value) = get::<i32>(arg_matches, "max_redirects") {
        options.push(HurlOption::new("max-redirs", value.to_string().as_str()));
    }
    if let Some(value) = get::<i32>(arg_matches, "retry") {
        options.push(HurlOption::new("retry", value.to_string().as_str()));
    }
    options
}

fn has_content_type(headers: &Vec<String>) -> bool {
    for header in headers {
        if header.starts_with("Content-Type") {
            return true;
        }
    }
    false
}

fn has_flag(matches: &ArgMatches, name: &str) -> bool {
    matches.get_one::<bool>(name) == Some(&true)
}

/// Returns an optional value of type `T` from the command line `matches` given the option `name`.
fn get<T: Clone + Send + Sync + 'static>(matches: &ArgMatches, name: &str) -> Option<T> {
    matches.get_one::<T>(name).cloned()
}

fn get_string(matches: &ArgMatches, name: &str) -> Option<String> {
    matches.get_one::<String>(name).map(|x| x.to_string())
}

/// Returns an optional list of `String` from the command line `matches` given the option `name`.
fn get_strings(matches: &ArgMatches, name: &str) -> Option<Vec<String>> {
    matches
        .get_many::<String>(name)
        .map(|v| v.map(|x| x.to_string()).collect())
}
