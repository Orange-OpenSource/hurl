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
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

use clap::ArgMatches;
use hurl::pretty::PrettyMode;
use hurl::runner::Value;
use hurl_core::input::Input;
use hurl_core::types::{BytesPerSec, Count, DurationUnit};

use super::context::RunContext;
use super::variables::TypeKind;
use super::variables_file::VariablesFile;
use super::{duration, variables, CliOptionsError, ErrorFormat, HttpVersion, IpResolve, Output};
use crate::cli::OutputType;

pub fn cacert_file(arg_matches: &ArgMatches) -> Result<Option<String>, CliOptionsError> {
    match get_string(arg_matches, "cacert_file") {
        None => Ok(None),
        Some(filename) => {
            let path = Path::new(&filename);
            if path.exists() {
                Ok(Some(filename))
            } else {
                Err(CliOptionsError::Error(format!(
                    "Input file {} does not exist",
                    path.display()
                )))
            }
        }
    }
}

pub fn aws_sigv4(arg_matches: &ArgMatches) -> Option<String> {
    get::<String>(arg_matches, "aws_sigv4")
}

pub fn client_cert_file(arg_matches: &ArgMatches) -> Result<Option<String>, CliOptionsError> {
    match get::<String>(arg_matches, "client_cert_file") {
        None => Ok(None),
        Some(filename) => {
            if !Path::new(&filename).is_file() {
                let message = format!("File {filename} does not exist");
                Err(CliOptionsError::Error(message))
            } else {
                Ok(Some(filename))
            }
        }
    }
}

pub fn client_key_file(arg_matches: &ArgMatches) -> Result<Option<String>, CliOptionsError> {
    match get::<String>(arg_matches, "client_key_file") {
        None => Ok(None),
        Some(filename) => {
            if !Path::new(&filename).is_file() {
                let message = format!("File {filename} does not exist");
                Err(CliOptionsError::Error(message))
            } else {
                Ok(Some(filename))
            }
        }
    }
}

/// Returns true if Hurl output uses ANSI code and false otherwise.
///
/// If it has no flags, we use the run `context` to determine if we use color or not.
pub fn color(arg_matches: &ArgMatches, context: &RunContext) -> bool {
    if has_flag(arg_matches, "color") {
        return true;
    }
    if has_flag(arg_matches, "no_color") {
        return false;
    }
    context.is_with_color()
}

pub fn compressed(arg_matches: &ArgMatches) -> bool {
    has_flag(arg_matches, "compressed")
}

pub fn connect_timeout(arg_matches: &ArgMatches) -> Result<Duration, CliOptionsError> {
    let s = get::<String>(arg_matches, "connect_timeout").unwrap_or("300".to_string());
    get_duration(&s, DurationUnit::Second)
}

pub fn connects_to(arg_matches: &ArgMatches) -> Vec<String> {
    get_strings(arg_matches, "connect_to").unwrap_or_default()
}

pub fn continue_on_error(arg_matches: &ArgMatches) -> bool {
    has_flag(arg_matches, "continue_on_error")
}

pub fn cookie_input_file(arg_matches: &ArgMatches) -> Option<String> {
    get::<String>(arg_matches, "cookies_input_file")
}

pub fn cookie_output_file(arg_matches: &ArgMatches) -> Option<PathBuf> {
    get::<String>(arg_matches, "cookies_output_file").map(PathBuf::from)
}

pub fn curl_file(arg_matches: &ArgMatches) -> Option<PathBuf> {
    get::<String>(arg_matches, "curl").map(PathBuf::from)
}

pub fn delay(arg_matches: &ArgMatches) -> Result<Duration, CliOptionsError> {
    let s = get::<String>(arg_matches, "delay").unwrap_or("0".to_string());
    get_duration(&s, DurationUnit::MilliSecond)
}

pub fn error_format(arg_matches: &ArgMatches) -> ErrorFormat {
    let error_format = get::<String>(arg_matches, "error_format").unwrap_or("short".to_string());
    match error_format.as_str() {
        "long" => ErrorFormat::Long,
        "short" => ErrorFormat::Short,
        _ => ErrorFormat::Short,
    }
}

pub fn file_root(arg_matches: &ArgMatches) -> Option<String> {
    get::<String>(arg_matches, "file_root")
}

pub fn follow_location(arg_matches: &ArgMatches) -> (bool, bool) {
    let follow_location = has_flag(arg_matches, "follow_location")
        || has_flag(arg_matches, "follow_location_trusted");
    let follow_location_trusted = has_flag(arg_matches, "follow_location_trusted");
    (follow_location, follow_location_trusted)
}

pub fn from_entry(arg_matches: &ArgMatches) -> Option<usize> {
    get::<u32>(arg_matches, "from_entry").map(|x| x as usize)
}

pub fn headers(arg_matches: &ArgMatches) -> Vec<String> {
    get_strings(arg_matches, "header").unwrap_or_default()
}

pub fn html_dir(arg_matches: &ArgMatches) -> Result<Option<PathBuf>, CliOptionsError> {
    if let Some(dir) = get::<String>(arg_matches, "report_html") {
        let path = Path::new(&dir);
        if !path.exists() {
            match fs::create_dir_all(path) {
                Err(_) => Err(CliOptionsError::Error(format!(
                    "HTML dir {} can not be created",
                    path.display()
                ))),
                Ok(_) => Ok(Some(path.to_path_buf())),
            }
        } else if path.is_dir() {
            Ok(Some(path.to_path_buf()))
        } else {
            Err(CliOptionsError::Error(format!(
                "{} is not a valid directory",
                path.display()
            )))
        }
    } else {
        Ok(None)
    }
}

pub fn http_version(arg_matches: &ArgMatches) -> Option<HttpVersion> {
    if has_flag(arg_matches, "http3") {
        Some(HttpVersion::V3)
    } else if has_flag(arg_matches, "http2") {
        Some(HttpVersion::V2)
    } else if has_flag(arg_matches, "http11") {
        Some(HttpVersion::V11)
    } else if has_flag(arg_matches, "http10") {
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

/// Returns true if we have at least one input files.
/// The input file can be a file, the standard input, or a glob (even a glob returns empty results).
pub fn has_input_files(arg_matches: &ArgMatches, context: &RunContext) -> bool {
    get_strings(arg_matches, "input_files").is_some()
        || get_strings(arg_matches, "glob").is_some()
        || !context.is_stdin_term()
}

/// Returns the input files from the positional arguments and the glob options
pub fn input_files(
    arg_matches: &ArgMatches,
    context: &RunContext,
) -> Result<Vec<Input>, CliOptionsError> {
    let mut files = vec![];
    if let Some(filenames) = get_strings(arg_matches, "input_files") {
        for filename in &filenames {
            let filename = Path::new(filename);
            if !filename.exists() {
                return Err(CliOptionsError::InvalidInputFile(filename.to_path_buf()));
            }
            if filename.is_file() {
                let file = Input::from(filename);
                files.push(file);
            } else if filename.is_dir() {
                walks_hurl_files(filename, &mut files)?;
            }
        }
    }
    for filename in glob_files(arg_matches)? {
        files.push(filename);
    }
    if files.is_empty() && !context.is_stdin_term() {
        let input = match Input::from_stdin() {
            Ok(input) => input,
            Err(err) => return Err(CliOptionsError::Error(err.to_string())),
        };
        files.push(input);
    }
    Ok(files)
}

/// Walks recursively a directory from `dir` and push Hurl files to `files`.
fn walks_hurl_files(dir: &Path, files: &mut Vec<Input>) -> Result<(), CliOptionsError> {
    let Ok(entries) = fs::read_dir(dir) else {
        return Err(CliOptionsError::InvalidInputFile(dir.to_path_buf()));
    };
    for entry in entries {
        let Ok(entry) = entry else {
            return Err(CliOptionsError::InvalidInputFile(dir.to_path_buf()));
        };
        let path = entry.path();
        if path.is_dir() {
            walks_hurl_files(&path, files)?;
        } else if entry.path().extension() == Some("hurl".as_ref()) {
            files.push(Input::from(entry.path()));
        }
    }
    Ok(())
}

pub fn insecure(arg_matches: &ArgMatches) -> bool {
    has_flag(arg_matches, "insecure")
}

pub fn interactive(arg_matches: &ArgMatches) -> bool {
    has_flag(arg_matches, "interactive")
}

pub fn ip_resolve(arg_matches: &ArgMatches) -> Option<IpResolve> {
    if has_flag(arg_matches, "ipv6") {
        Some(IpResolve::IpV6)
    } else if has_flag(arg_matches, "ipv4") {
        Some(IpResolve::IpV4)
    } else {
        None
    }
}

pub fn junit_file(arg_matches: &ArgMatches) -> Option<PathBuf> {
    get::<String>(arg_matches, "report_junit").map(PathBuf::from)
}

pub fn limit_rate(arg_matches: &ArgMatches) -> Option<BytesPerSec> {
    get::<u64>(arg_matches, "limit_rate").map(BytesPerSec)
}

pub fn max_filesize(arg_matches: &ArgMatches) -> Option<u64> {
    get::<u64>(arg_matches, "max_filesize")
}

pub fn max_redirect(arg_matches: &ArgMatches) -> Count {
    match get::<i32>(arg_matches, "max_redirects").unwrap_or(50) {
        -1 => Count::Infinite,
        m => Count::Finite(m as usize),
    }
}

pub fn jobs(arg_matches: &ArgMatches) -> Option<usize> {
    get::<u32>(arg_matches, "jobs").map(|m| m as usize)
}

pub fn json_report_dir(arg_matches: &ArgMatches) -> Result<Option<PathBuf>, CliOptionsError> {
    if let Some(dir) = get::<String>(arg_matches, "report_json") {
        let path = Path::new(&dir);
        if !path.exists() {
            match fs::create_dir_all(path) {
                Err(_) => Err(CliOptionsError::Error(format!(
                    "JSON dir {} can not be created",
                    path.display()
                ))),
                Ok(_) => Ok(Some(path.to_path_buf())),
            }
        } else if path.is_dir() {
            Ok(Some(path.to_path_buf()))
        } else {
            Err(CliOptionsError::Error(format!(
                "{} is not a valid directory",
                path.display()
            )))
        }
    } else {
        Ok(None)
    }
}

pub fn negotiate(arg_matches: &ArgMatches) -> bool {
    has_flag(arg_matches, "negotiate")
}

pub fn netrc(arg_matches: &ArgMatches) -> bool {
    has_flag(arg_matches, "netrc")
}

pub fn netrc_file(arg_matches: &ArgMatches) -> Result<Option<String>, CliOptionsError> {
    match get::<String>(arg_matches, "netrc_file") {
        None => Ok(None),
        Some(filename) => {
            if !Path::new(&filename).is_file() {
                let message = format!("File {filename} does not exist");
                Err(CliOptionsError::Error(message))
            } else {
                Ok(Some(filename))
            }
        }
    }
}

pub fn netrc_optional(arg_matches: &ArgMatches) -> bool {
    has_flag(arg_matches, "netrc_optional")
}

pub fn no_proxy(arg_matches: &ArgMatches) -> Option<String> {
    get::<String>(arg_matches, "noproxy")
}

pub fn ntlm(arg_matches: &ArgMatches) -> bool {
    has_flag(arg_matches, "ntlm")
}

pub fn output(arg_matches: &ArgMatches) -> Option<Output> {
    get::<String>(arg_matches, "output").map(|filename| Output::new(&filename))
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

pub fn parallel(arg_matches: &ArgMatches) -> bool {
    has_flag(arg_matches, "parallel") || has_flag(arg_matches, "test")
}

pub fn path_as_is(arg_matches: &ArgMatches) -> bool {
    has_flag(arg_matches, "path_as_is")
}

pub fn pinned_pub_key(arg_matches: &ArgMatches) -> Option<String> {
    get::<String>(arg_matches, "pinned_pub_key")
}

pub fn pretty(arg_matches: &ArgMatches, context: &RunContext) -> PrettyMode {
    if has_flag(arg_matches, "pretty") {
        return PrettyMode::Force;
    }
    if has_flag(arg_matches, "no_pretty") {
        return PrettyMode::None;
    }
    if context.is_stdout_term() {
        PrettyMode::Automatic
    } else {
        PrettyMode::None
    }
}

pub fn progress_bar(arg_matches: &ArgMatches, context: &RunContext) -> bool {
    // The test progress bar is displayed only for in test mode, for interactive TTYs.
    // It can be forced by `--progress-bar` option.
    if !test(arg_matches) {
        return false;
    }
    if has_flag(arg_matches, "progress_bar") {
        return true;
    }
    context.is_stderr_term() && !context.is_ci()
}

pub fn proxy(arg_matches: &ArgMatches) -> Option<String> {
    get::<String>(arg_matches, "proxy")
}

pub fn repeat(arg_matches: &ArgMatches) -> Option<Count> {
    match get::<i32>(arg_matches, "repeat") {
        Some(-1) => Some(Count::Infinite),
        Some(n) => Some(Count::Finite(n as usize)),
        None => None,
    }
}

pub fn resolves(arg_matches: &ArgMatches) -> Vec<String> {
    get_strings(arg_matches, "resolve").unwrap_or_default()
}

pub fn retry(arg_matches: &ArgMatches) -> Option<Count> {
    match get::<i32>(arg_matches, "retry") {
        Some(-1) => Some(Count::Infinite),
        Some(r) => Some(Count::Finite(r as usize)),
        None => None,
    }
}

pub fn retry_interval(arg_matches: &ArgMatches) -> Result<Duration, CliOptionsError> {
    let s = get::<String>(arg_matches, "retry_interval").unwrap_or("1000".to_string());
    get_duration(&s, DurationUnit::MilliSecond)
}

pub fn secret(
    matches: &ArgMatches,
    context: &RunContext,
) -> Result<HashMap<String, String>, CliOptionsError> {
    let mut all_secrets = HashMap::new();

    // Secrets are always parsed as string.
    let type_kind = TypeKind::String;

    // Insert environment secrets `HURL_SECRET_foo`
    for (env_name, env_value) in context.secret_env_vars() {
        let value = variables::parse_value(env_value, type_kind)?;
        add_secret(&mut all_secrets, env_name.to_string(), value)?;
    }

    // Add secrets from files:
    if let Some(filenames) = get_strings(matches, "secrets_file") {
        for f in &filenames {
            let filename = Path::new(f);
            let vars = VariablesFile::open(filename, type_kind)?;
            for var in vars {
                let (name, value) = var?;
                add_secret(&mut all_secrets, name, value)?;
            }
        }
    }

    // Finally, add single secrets.
    if let Some(secrets) = get_strings(matches, "secret") {
        for s in secrets {
            let (name, value) = variables::parse(&s, type_kind)?;
            add_secret(&mut all_secrets, name, value)?;
        }
    }
    Ok(all_secrets)
}

/// Add a secret with `name` and `value` to the `secrets` hash map.
fn add_secret(
    secrets: &mut HashMap<String, String>,
    name: String,
    value: Value,
) -> Result<(), CliOptionsError> {
    // We check that there is no existing secrets
    if secrets.contains_key(&name) {
        return Err(CliOptionsError::Error(format!(
            "secret '{}' can't be reassigned",
            &name
        )));
    }
    // Secrets can only be string.
    if let Value::String(value) = value {
        secrets.insert(name.to_string(), value);
    }
    Ok(())
}

pub fn ssl_no_revoke(arg_matches: &ArgMatches) -> bool {
    has_flag(arg_matches, "ssl_no_revoke")
}

pub fn tap_file(arg_matches: &ArgMatches) -> Option<PathBuf> {
    get::<String>(arg_matches, "report_tap").map(PathBuf::from)
}

pub fn test(arg_matches: &ArgMatches) -> bool {
    has_flag(arg_matches, "test")
}

pub fn timeout(arg_matches: &ArgMatches) -> Result<Duration, CliOptionsError> {
    let s = get::<String>(arg_matches, "max_time").unwrap_or("300".to_string());
    get_duration(&s, DurationUnit::Second)
}

pub fn to_entry(arg_matches: &ArgMatches) -> Option<usize> {
    get::<u32>(arg_matches, "to_entry").map(|x| x as usize)
}

pub fn unix_socket(arg_matches: &ArgMatches) -> Option<String> {
    get::<String>(arg_matches, "unix_socket")
}

pub fn user(arg_matches: &ArgMatches) -> Option<String> {
    get::<String>(arg_matches, "user")
}

pub fn user_agent(arg_matches: &ArgMatches) -> Option<String> {
    get::<String>(arg_matches, "user_agent")
}

/// Returns a map of variables from the command line options `matches`.
pub fn variables(
    matches: &ArgMatches,
    context: &RunContext,
) -> Result<HashMap<String, Value>, CliOptionsError> {
    let mut variables = HashMap::new();

    // Variables are typed, based on their values.
    let type_kind = TypeKind::Inferred;

    // Insert environment variables `HURL_VARIABLE_foo`
    for (env_name, env_value) in context.var_env_vars() {
        let value = variables::parse_value(env_value, type_kind)?;
        variables.insert(env_name.to_string(), value);
    }

    // Insert legacy environment variables `HURL_foo`
    for (env_name, env_value) in context.legacy_var_env_vars() {
        let value = variables::parse_value(env_value, type_kind)?;
        variables.insert(env_name.to_string(), value);
    }

    // Then add variables from files:
    if let Some(filenames) = get_strings(matches, "variables_file") {
        for f in &filenames {
            let filename = Path::new(f);
            let vars = VariablesFile::open(filename, type_kind)?;
            for var in vars {
                let (name, value) = var?;
                variables.insert(name.to_string(), value);
            }
        }
    }

    // Finally, add single variables from command line.
    if let Some(input) = get_strings(matches, "variable") {
        for s in input {
            let (name, value) = variables::parse(&s, type_kind)?;
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
fn glob_files(matches: &ArgMatches) -> Result<Vec<Input>, CliOptionsError> {
    let mut all_files = vec![];
    if let Some(exprs) = get_strings(matches, "glob") {
        for expr in exprs {
            let paths = match glob::glob(&expr) {
                Ok(paths) => paths,
                Err(_) => {
                    return Err(CliOptionsError::Error(
                        "Failed to read glob pattern".to_string(),
                    ))
                }
            };
            let mut files = vec![];
            for entry in paths {
                match entry {
                    Ok(path) => files.push(Input::from(path)),
                    Err(_) => {
                        return Err(CliOptionsError::Error(
                            "Failed to read glob pattern".to_string(),
                        ))
                    }
                }
            }
            if files.is_empty() {
                return Err(CliOptionsError::InvalidInputFile(PathBuf::from(&expr)));
            }
            all_files.extend(files);
        }
    }
    Ok(all_files)
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

/// Get duration from input string `s` and `default_unit`
fn get_duration(s: &str, default_unit: DurationUnit) -> Result<Duration, CliOptionsError> {
    let duration = duration::parse(s).map_err(CliOptionsError::Error)?;
    let unit = duration.unit.unwrap_or(default_unit);
    let millis = match unit {
        DurationUnit::MilliSecond => duration.value.as_u64(),
        DurationUnit::Second => duration.value.as_u64() * 1000,
        DurationUnit::Minute => duration.value.as_u64() * 1000 * 60,
        DurationUnit::Hour => duration.value.as_u64() * 1000 * 60 * 60,
    };
    Ok(Duration::from_millis(millis))
}
