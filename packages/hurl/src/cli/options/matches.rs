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
use super::variables::{parse as parse_variable, parse_value};
use super::OptionsError;
use crate::cli::options::{ErrorFormat, HttpVersion};
use crate::cli::OutputType;
use clap::ArgMatches;
use hurl::runner::Value;
use hurl_core::ast::Retry;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, IsTerminal};
use std::path::{Path, PathBuf};
use std::time::Duration;
use std::{env, io};

pub fn cacert_file(arg_matches: &ArgMatches) -> Result<Option<String>, OptionsError> {
    match get_string(arg_matches, "cacert_file") {
        None => Ok(None),
        Some(filename) => {
            let path = Path::new(&filename);
            if path.exists() {
                Ok(Some(filename))
            } else {
                Err(OptionsError::Error(format!(
                    "input file {} does not exist",
                    path.display()
                )))
            }
        }
    }
}

pub fn aws_sigv4(arg_matches: &ArgMatches) -> Option<String> {
    get::<String>(arg_matches, "aws_sigv4")
}

pub fn client_cert_file(arg_matches: &ArgMatches) -> Result<Option<String>, OptionsError> {
    match get::<String>(arg_matches, "client_cert_file") {
        None => Ok(None),
        Some(filename) => {
            if !Path::new(&filename).is_file() {
                let message = format!("File {filename} does not exist");
                Err(OptionsError::Error(message))
            } else {
                Ok(Some(filename))
            }
        }
    }
}

pub fn client_key_file(arg_matches: &ArgMatches) -> Result<Option<String>, OptionsError> {
    match get::<String>(arg_matches, "client_key_file") {
        None => Ok(None),
        Some(filename) => {
            if !Path::new(&filename).is_file() {
                let message = format!("File {filename} does not exist");
                Err(OptionsError::Error(message))
            } else {
                Ok(Some(filename))
            }
        }
    }
}

/// Returns true if Hurl output uses ANSI code and false otherwise.
pub fn color(arg_matches: &ArgMatches) -> bool {
    if has_flag(arg_matches, "color") {
        return true;
    }
    if has_flag(arg_matches, "no_color") {
        return false;
    }
    if let Ok(v) = env::var("NO_COLOR") {
        if !v.is_empty() {
            return false;
        }
    }
    io::stdout().is_terminal()
}

pub fn compressed(arg_matches: &ArgMatches) -> bool {
    has_flag(arg_matches, "compressed")
}

pub fn connect_timeout(arg_matches: &ArgMatches) -> Duration {
    let value = get::<u64>(arg_matches, "connect_timeout").unwrap();
    Duration::from_secs(value)
}

pub fn connects_to(arg_matches: &ArgMatches) -> Vec<String> {
    get_strings(arg_matches, "connect_to").unwrap_or_default()
}

pub fn continue_on_error(arg_matches: &ArgMatches) -> bool {
    if has_flag(arg_matches, "fail_at_end") {
        eprintln!("The option fail-at-end is deprecated. Use continue-on-error instead");
        return true;
    }
    has_flag(arg_matches, "continue_on_error")
}

pub fn cookie_input_file(arg_matches: &ArgMatches) -> Option<String> {
    get::<String>(arg_matches, "cookies_input_file")
}

pub fn cookie_output_file(arg_matches: &ArgMatches) -> Option<String> {
    get::<String>(arg_matches, "cookies_output_file")
}

pub fn delay(arg_matches: &ArgMatches) -> Duration {
    let millis = get::<u64>(arg_matches, "delay").unwrap();
    Duration::from_millis(millis)
}

pub fn error_format(arg_matches: &ArgMatches) -> ErrorFormat {
    let error_format = get::<String>(arg_matches, "error_format");
    match error_format.as_deref() {
        Some("long") => ErrorFormat::Long,
        Some("short") => ErrorFormat::Short,
        _ => ErrorFormat::Short,
    }
}

pub fn file_root(arg_matches: &ArgMatches) -> Option<String> {
    get::<String>(arg_matches, "file_root")
}

pub fn follow_location(arg_matches: &ArgMatches) -> bool {
    has_flag(arg_matches, "follow_location")
}

pub fn html_dir(arg_matches: &ArgMatches) -> Result<Option<PathBuf>, OptionsError> {
    if let Some(dir) = get::<String>(arg_matches, "report_html") {
        let path = Path::new(&dir);
        if !path.exists() {
            match std::fs::create_dir(path) {
                Err(_) => Err(OptionsError::Error(format!(
                    "HTML dir {} can not be created",
                    path.display()
                ))),
                Ok(_) => Ok(Some(path.to_path_buf())),
            }
        } else if path.is_dir() {
            Ok(Some(path.to_path_buf()))
        } else {
            return Err(OptionsError::Error(format!(
                "{} is not a valid directory",
                path.display()
            )));
        }
    } else {
        Ok(None)
    }
}

pub fn http_version(arg_matches: &ArgMatches) -> Option<HttpVersion> {
    if has_flag(arg_matches, "http10") {
        Some(HttpVersion::V10)
    } else {
        None
    }
}

pub fn ignore_asserts(arg_matches: &ArgMatches) -> bool {
    has_flag(arg_matches, "ignore_asserts")
}

pub fn include(arg_matches: &ArgMatches) -> bool {
    has_flag(arg_matches, "include")
}

/// Returns the input files from the positional arguments and the glob options
pub fn input_files(arg_matches: &ArgMatches) -> Result<Vec<String>, OptionsError> {
    let mut files = vec![];
    if let Some(filenames) = get_strings(arg_matches, "input_files") {
        for filename in filenames {
            let path = Path::new(&filename);
            if path.exists() {
                files.push(filename);
            } else {
                return Err(OptionsError::Error(format!(
                    "hurl: cannot access '{}': No such file or directory",
                    path.display()
                )));
            }
        }
    }
    for filename in glob_files(arg_matches)? {
        files.push(filename);
    }
    if files.is_empty() && !io::stdin().is_terminal() {
        files.push("-".to_string());
    }
    Ok(files)
}

pub fn insecure(arg_matches: &ArgMatches) -> bool {
    has_flag(arg_matches, "insecure")
}

pub fn interactive(arg_matches: &ArgMatches) -> bool {
    has_flag(arg_matches, "interactive")
}

pub fn junit_file(arg_matches: &ArgMatches) -> Option<String> {
    get::<String>(arg_matches, "junit")
}

pub fn max_redirect(arg_matches: &ArgMatches) -> Option<usize> {
    match get::<i32>(arg_matches, "max_redirects").unwrap() {
        m if m == -1 => None,
        m => Some(m as usize),
    }
}

pub fn no_proxy(arg_matches: &ArgMatches) -> Option<String> {
    get::<String>(arg_matches, "noproxy")
}

pub fn output(arg_matches: &ArgMatches) -> Option<String> {
    get::<String>(arg_matches, "output")
}

pub fn output_type(arg_matches: &ArgMatches) -> OutputType {
    if has_flag(arg_matches, "json") {
        OutputType::Json
    } else if has_flag(arg_matches, "no_output") || test(arg_matches) {
        OutputType::NoOutput
    } else {
        OutputType::ResponseBody
    }
}

pub fn path_as_is(arg_matches: &ArgMatches) -> bool {
    has_flag(arg_matches, "path_as_is")
}

pub fn progress_bar(arg_matches: &ArgMatches) -> bool {
    let verbose = verbose(arg_matches) || very_verbose(arg_matches);
    test(arg_matches)
        && !verbose
        && !interactive(arg_matches)
        && !is_ci()
        && io::stderr().is_terminal()
}

pub fn proxy(arg_matches: &ArgMatches) -> Option<String> {
    get::<String>(arg_matches, "proxy")
}

pub fn resolves(arg_matches: &ArgMatches) -> Vec<String> {
    get_strings(arg_matches, "resolve").unwrap_or_default()
}

pub fn retry(arg_matches: &ArgMatches) -> Retry {
    match get::<i32>(arg_matches, "retry").unwrap() {
        r if r == -1 => Retry::Infinite,
        r if r == 0 => Retry::None,
        r => Retry::Finite(r as usize),
    }
}

pub fn retry_interval(arg_matches: &ArgMatches) -> Duration {
    let value = get::<u64>(arg_matches, "retry_interval").unwrap();
    Duration::from_millis(value)
}

pub fn ssl_no_revoke(arg_matches: &ArgMatches) -> bool {
    has_flag(arg_matches, "ssl_no_revoke")
}

pub fn tap_file(arg_matches: &ArgMatches) -> Option<String> {
    get::<String>(arg_matches, "tap")
}

pub fn test(arg_matches: &ArgMatches) -> bool {
    has_flag(arg_matches, "test")
}

pub fn timeout(arg_matches: &ArgMatches) -> Duration {
    let value = get::<u64>(arg_matches, "max_time").unwrap();
    Duration::from_secs(value)
}

pub fn to_entry(arg_matches: &ArgMatches) -> Option<usize> {
    get::<u32>(arg_matches, "to_entry").map(|x| x as usize)
}

pub fn user(arg_matches: &ArgMatches) -> Option<String> {
    get::<String>(arg_matches, "user")
}

pub fn user_agent(arg_matches: &ArgMatches) -> Option<String> {
    get::<String>(arg_matches, "user_agent")
}

/// Returns a map of variables from the command line options `matches`.
pub fn variables(matches: &ArgMatches) -> Result<HashMap<String, Value>, OptionsError> {
    let mut variables = HashMap::new();

    // Use environment variables prefix by HURL_
    for (env_name, env_value) in env::vars() {
        if let Some(name) = env_name.strip_prefix("HURL_") {
            let value = parse_value(env_value.as_str())?;
            variables.insert(name.to_string(), value);
        }
    }

    if let Some(filenames) = get_strings(matches, "variables_file") {
        for f in filenames.iter() {
            let path = Path::new(&f);
            if !path.exists() {
                return Err(OptionsError::Error(format!(
                    "Properties file {} does not exist",
                    path.display()
                )));
            }

            let file = File::open(path).unwrap();
            let reader = BufReader::new(file);
            for (index, line) in reader.lines().enumerate() {
                let line = match line {
                    Ok(s) => s,
                    Err(_) => {
                        return Err(OptionsError::Error(format!(
                            "Can not parse line {} of {}",
                            index + 1,
                            path.display()
                        )))
                    }
                };
                let line = line.trim();
                if line.starts_with('#') || line.is_empty() {
                    continue;
                }
                let (name, value) = parse_variable(line)?;
                variables.insert(name.to_string(), value);
            }
        }
    }

    if let Some(input) = get_strings(matches, "variable") {
        for s in input {
            let (name, value) = parse_variable(&s)?;
            variables.insert(name.to_string(), value);
        }
    }

    Ok(variables)
}

pub fn verbose(arg_matches: &ArgMatches) -> bool {
    has_flag(arg_matches, "verbose")
}

pub fn very_verbose(arg_matches: &ArgMatches) -> bool {
    has_flag(arg_matches, "very_verbose")
}

/// Returns a list of path names from the command line options `matches`.
fn glob_files(matches: &ArgMatches) -> Result<Vec<String>, OptionsError> {
    let mut filenames = vec![];
    if let Some(exprs) = get_strings(matches, "glob") {
        for expr in exprs {
            let paths = match glob::glob(&expr) {
                Ok(paths) => paths,
                Err(_) => {
                    return Err(OptionsError::Error(
                        "Failed to read glob pattern".to_string(),
                    ))
                }
            };
            for entry in paths {
                match entry {
                    Ok(path) => match path.into_os_string().into_string() {
                        Ok(filename) => filenames.push(filename),
                        Err(_) => {
                            return Err(OptionsError::Error(
                                "Failed to read glob pattern".to_string(),
                            ))
                        }
                    },
                    Err(_) => {
                        return Err(OptionsError::Error(
                            "Failed to read glob pattern".to_string(),
                        ))
                    }
                }
            }
            if filenames.is_empty() {
                let message = format!("hurl: cannot access '{expr}': No such file or directory");
                return Err(OptionsError::Error(message));
            }
        }
    }
    Ok(filenames)
}

/// Returns an optional value of type `T` from the command line `matches` given the option `name`.
fn get<T: Clone + Send + Sync + 'static>(matches: &ArgMatches, name: &str) -> Option<T> {
    matches.get_one::<T>(name).cloned()
}

fn has_flag(matches: &ArgMatches, name: &str) -> bool {
    matches.get_one::<bool>(name) == Some(&true)
}

fn get_string(matches: &ArgMatches, name: &str) -> Option<String> {
    matches.get_one::<String>(name).map(|x| x.to_string())
}

/// Returns an optional list of `String` from the command line `matches` given the option `name`.
pub fn get_strings(matches: &ArgMatches, name: &str) -> Option<Vec<String>> {
    matches
        .get_many::<String>(name)
        .map(|v| v.map(|x| x.to_string()).collect())
}

/// Whether or not this running in a Continuous Integration environment.
/// Code borrowed from <https://github.com/rust-lang/cargo/blob/master/crates/cargo-util/src/lib.rs>
fn is_ci() -> bool {
    env::var("CI").is_ok() || env::var("TF_BUILD").is_ok()
}
