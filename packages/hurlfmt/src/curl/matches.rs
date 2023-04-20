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
use clap::ArgMatches;

pub fn method(arg_matches: &ArgMatches) -> String {
    match get_string(arg_matches, "method") {
        None => "GET".to_string(),
        Some(v) => v,
    }
}

pub fn url(arg_matches: &ArgMatches) -> String {
    let s = get_string(arg_matches, "url").unwrap();
    if !s.starts_with("http") {
        format!("https://{s}")
    } else {
        s
    }
}

pub fn headers(arg_matches: &ArgMatches) -> Vec<String> {
    match get_strings(arg_matches, "headers") {
        None => vec![],
        Some(v) => v,
    }
}

pub fn get_string(matches: &ArgMatches, name: &str) -> Option<String> {
    matches.get_one::<String>(name).map(|x| x.to_string())
}

/// Returns an optional list of `String` from the command line `matches` given the option `name`.
pub fn get_strings(matches: &ArgMatches, name: &str) -> Option<Vec<String>> {
    matches
        .get_many::<String>(name)
        .map(|v| v.map(|x| x.to_string()).collect())
}
