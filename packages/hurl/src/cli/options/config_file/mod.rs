/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2026 Orange
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
mod primitives;

use std::path::Path;

use hurl_core::reader::{CharPos, Pos, Reader};
use hurl_core::types::DurationUnit;

use crate::cli::options::config_file::primitives::{
    expect_no_value, parse_value, parse_value_separator,
};
use crate::cli::options::duration;

use super::{CliOptions, CliOptionsError, IpResolve, OutputType, Verbosity};
use hurl_core::types::{BytesPerSec, Count};
use primitives::skip_whitespace_and_comments;

#[derive(Debug, PartialEq, Eq)]
struct ConfigFileError {
    pos: Pos,
    message: String,
}

impl ConfigFileError {
    fn new(pos: Pos, message: &str) -> Self {
        ConfigFileError {
            pos,
            message: message.to_string(),
        }
    }
    fn pos(&self) -> Pos {
        self.pos
    }
}

impl std::fmt::Display for ConfigFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}: {}", self.pos.line, self.pos.column, self.message)
    }
}

/// Parse options from config file `config_file_path`, using `default_options` for default values.
pub fn parse_config_file(
    config_file_path: Option<&Path>,
    default_options: CliOptions,
) -> Result<CliOptions, CliOptionsError> {
    if let Some(config_file_path) = config_file_path
        && config_file_path.exists()
    {
        let content = std::fs::read_to_string(config_file_path).map_err(|e| {
            CliOptionsError::Error(format!(
                "Failed to read config file {}: {}",
                config_file_path.display(),
                e
            ))
        })?;
        return parse_config(&content, default_options).map_err(|e| {
            CliOptionsError::Error(format!(
                "{}:{}:{}: {}",
                config_file_path.display(),
                e.pos().line,
                e.pos().column,
                e.message
            ))
        });
    }
    Ok(default_options)
}

/// Parse config `content` using `default_options`
fn parse_config(content: &str, default_options: CliOptions) -> Result<CliOptions, ConfigFileError> {
    let mut options = default_options;
    let mut reader = Reader::new(content);
    while !reader.is_eof() {
        parse_option(&mut reader, &mut options)?;
    }
    Ok(options)
}

/// Parse and process a single option from config file
/// Empty lines or lines starting with `#` are ignored
fn parse_option(reader: &mut Reader, options: &mut CliOptions) -> Result<(), ConfigFileError> {
    skip_whitespace_and_comments(reader);
    if reader.is_eof() {
        return Ok(());
    }

    let mut save = reader.cursor();
    if reader.read_n(CharPos(2)) != "--" {
        return Err(ConfigFileError::new(
            save.pos,
            "Expecting an option starting with --",
        ));
    }

    let option_name = reader.read_while(|c: char| c.is_alphanumeric() || c == '-' || c == '_');
    match option_name.as_str() {
        "verbose" => {
            expect_no_value(reader)?;
            options.verbosity = Some(Verbosity::Verbose);
            Ok(())
        }
        "header" => {
            parse_value_separator(reader)?;
            let value = parse_value(reader)?;

            if value.is_empty() {
                return Err(ConfigFileError::new(
                    save.pos,
                    "Option --header requires a value",
                ));
            }
            options.headers.push(value);

            Ok(())
        }
        "max-redirs" => {
            parse_value_separator(reader)?;
            save = reader.cursor();
            let value = parse_value(reader)?;
            let max_redirs = value.parse::<i32>().map_err(|_| {
                ConfigFileError::new(save.pos, "Option --max-redirs requires an integer value")
            })?;
            options.max_redirect = Count::try_from(max_redirs).map_err(|_| {
                ConfigFileError::new(
                    save.pos,
                    "Option --max-redirs requires an integer value >= -1",
                )
            })?;

            Ok(())
        }
        "delay" => {
            parse_value_separator(reader)?;
            save = reader.cursor();
            let value = parse_value(reader)?;
            options.delay = duration::duration_from_str(&value, DurationUnit::MilliSecond)
                .map_err(|_| {
                    ConfigFileError::new(save.pos, "Option --delay has an invalid duration")
                })?;
            Ok(())
        }
        "limit-rate" => {
            parse_value_separator(reader)?;
            save = reader.cursor();
            let value = parse_value(reader)?;
            let limit_rate = value.parse::<u64>().map_err(|_| {
                ConfigFileError::new(save.pos, "Option --limit-rate requires an integer value")
            })?;
            options.limit_rate = Some(BytesPerSec(limit_rate));
            Ok(())
        }
        "insecure" => {
            expect_no_value(reader)?;
            options.insecure = true;
            Ok(())
        }
        "location" => {
            expect_no_value(reader)?;
            options.follow_location = true;
            Ok(())
        }
        "location-trusted" => {
            expect_no_value(reader)?;
            options.follow_location = true;
            options.follow_location_trusted = true;
            Ok(())
        }
        "ipv6" => {
            expect_no_value(reader)?;
            options.ip_resolve = Some(IpResolve::IpV6);
            Ok(())
        }
        "compressed" => {
            expect_no_value(reader)?;
            options.compressed = true;
            Ok(())
        }
        "fail-with-body" => {
            expect_no_value(reader)?;
            options.fail_with_body = true;
            Ok(())
        }
        "no-assert" => {
            expect_no_value(reader)?;
            options.no_assert = true;
            Ok(())
        }
        "no-output" => {
            expect_no_value(reader)?;
            options.output_type = OutputType::NoOutput;
            Ok(())
        }
        "user-agent" => {
            parse_value_separator(reader)?;
            let value = parse_value(reader)?;
            options.user_agent = Some(value);
            Ok(())
        }
        "color" => {
            expect_no_value(reader)?;
            options.color_stdout = true;
            options.color_stderr = true;
            Ok(())
        }
        "no-color" => {
            expect_no_value(reader)?;
            options.color_stdout = false;
            options.color_stderr = false;
            Ok(())
        }
        _ => Err(ConfigFileError::new(
            save.pos,
            &format!("Unknown option <--{}>", option_name),
        )),
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::cli::options::Verbosity;
    use hurl_core::reader::Pos;

    #[test]
    fn test_parse_config() {
        let content = "# ignore\n\n--verbose\n";
        let options = parse_config(content, CliOptions::default()).unwrap();
        assert!(!options.color_stdout);
        assert!(!options.color_stderr);
        assert_eq!(options.verbosity, Some(Verbosity::Verbose));
    }

    #[test]
    fn test_parse_option_flag() {
        let mut reader = Reader::new("\n\n--verbose\n");
        let mut options = CliOptions::default();
        assert_eq!(reader.cursor().pos, Pos::new(1, 1));
        assert!(parse_option(&mut reader, &mut options).is_ok());
        assert_eq!(options.verbosity, Some(Verbosity::Verbose));
        assert_eq!(reader.cursor().pos, Pos::new(4, 1));
    }

    #[test]
    fn test_parse_no_option() {
        let mut reader = Reader::new("# ignore\n\n");
        let mut options = CliOptions::default();
        assert!(parse_option(&mut reader, &mut options).is_ok());
        assert_eq!(reader.cursor().pos, Pos::new(3, 1));
    }

    #[test]
    fn test_parse_option_with_value() {
        let mut reader = Reader::new("--header=header1:value1\n--verbose\n");
        let mut options = CliOptions::default();
        assert!(parse_option(&mut reader, &mut options).is_ok());
        assert_eq!(options.headers, vec!["header1:value1"]);
        assert_eq!(reader.cursor().pos, Pos::new(1, 24));

        let mut reader = Reader::new("--header header2:value2\n--verbose\n");
        let mut options = CliOptions::default();
        assert!(parse_option(&mut reader, &mut options).is_ok());
        assert_eq!(options.headers, vec!["header2:value2"]);
        assert_eq!(reader.cursor().pos, Pos::new(1, 24));

        let mut reader = Reader::new("--header --test:1\n");
        let mut options = CliOptions::default();
        assert!(parse_option(&mut reader, &mut options).is_ok());
        assert_eq!(options.headers, vec!["--test:1"]);
        assert_eq!(reader.cursor().pos, Pos::new(1, 18));
    }

    #[test]
    fn test_parse_option_with_value_with_quotes() {
        let mut reader = Reader::new("--user-agent=\"Mozilla/5.0 A\"");
        let mut options = CliOptions::default();
        assert!(parse_option(&mut reader, &mut options).is_ok());
        assert_eq!(options.user_agent.unwrap(), "Mozilla/5.0 A".to_string());
        assert_eq!(reader.cursor().pos, Pos::new(1, 29));
    }

    #[test]
    fn test_parse_option_delay() {
        let mut reader = Reader::new("--delay=1s\n");
        let mut options = CliOptions::default();
        assert!(parse_option(&mut reader, &mut options).is_ok());
        assert_eq!(options.delay, std::time::Duration::from_secs(1));
        assert_eq!(reader.cursor().pos, Pos::new(1, 11));
    }

    #[test]
    fn test_parse_option_limit_rate() {
        let mut reader = Reader::new("--limit-rate=2000000\n");
        let mut options = CliOptions::default();
        assert!(parse_option(&mut reader, &mut options).is_ok());
        assert_eq!(options.limit_rate, Some(BytesPerSec(2_000_000)));
        assert_eq!(reader.cursor().pos, Pos::new(1, 21));
    }

    #[test]
    fn test_parse_option_color() {
        let mut reader = Reader::new("--color\n");
        let mut options = CliOptions::default();
        assert!(!options.color_stdout);
        assert!(!options.color_stderr);
        assert!(parse_option(&mut reader, &mut options).is_ok());
        assert!(options.color_stdout);
        assert!(options.color_stderr);
        assert_eq!(reader.cursor().pos, Pos::new(2, 1));
    }

    #[test]
    fn test_parse_option_no_color() {
        let mut reader = Reader::new("--no-color\n");
        let mut options = CliOptions {
            color_stdout: true,
            color_stderr: true,
            ..CliOptions::default()
        };
        assert!(parse_option(&mut reader, &mut options).is_ok());
        assert!(!options.color_stdout);
        assert!(!options.color_stderr);
        assert_eq!(reader.cursor().pos, Pos::new(2, 1));
    }

    #[test]
    fn test_parse_option_insecure() {
        let mut reader = Reader::new("--insecure\n");
        let mut options = CliOptions::default();
        assert!(!options.insecure);
        assert!(parse_option(&mut reader, &mut options).is_ok());
        assert!(options.insecure);
        assert_eq!(reader.cursor().pos, Pos::new(2, 1));
    }

    #[test]
    fn test_parse_option_follow_location() {
        let mut reader = Reader::new("--location\n");
        let mut options = CliOptions::default();
        assert!(!options.follow_location);
        assert!(parse_option(&mut reader, &mut options).is_ok());
        assert!(options.follow_location);
        assert_eq!(reader.cursor().pos, Pos::new(2, 1));
    }

    #[test]
    fn test_parse_option_follow_location_trusted() {
        let mut reader = Reader::new("--location-trusted\n");
        let mut options = CliOptions::default();
        assert!(!options.follow_location);
        assert!(!options.follow_location_trusted);
        assert!(parse_option(&mut reader, &mut options).is_ok());
        assert!(options.follow_location);
        assert!(options.follow_location_trusted);
        assert_eq!(reader.cursor().pos, Pos::new(2, 1));
    }

    #[test]
    fn test_parse_option_compressed() {
        let mut reader = Reader::new("--compressed\n");
        let mut options = CliOptions::default();
        assert!(!options.compressed);
        assert!(parse_option(&mut reader, &mut options).is_ok());
        assert!(options.compressed);
        assert_eq!(reader.cursor().pos, Pos::new(2, 1));
    }

    #[test]
    fn test_parse_option_ipv6() {
        let mut reader = Reader::new("--ipv6\n");
        let mut options = CliOptions::default();
        assert_eq!(options.ip_resolve, None);
        assert!(parse_option(&mut reader, &mut options).is_ok());
        assert_eq!(options.ip_resolve, Some(IpResolve::IpV6));
        assert_eq!(reader.cursor().pos, Pos::new(2, 1));
    }

    #[test]
    fn test_parse_option_fail_with_body() {
        let mut reader = Reader::new("--fail-with-body\n");
        let mut options = CliOptions::default();
        assert!(!options.fail_with_body);
        assert!(parse_option(&mut reader, &mut options).is_ok());
        assert!(options.fail_with_body);
        assert_eq!(reader.cursor().pos, Pos::new(2, 1));
    }

    #[test]
    fn test_parse_option_no_output() {
        let mut reader = Reader::new("--no-output\n");
        let mut options = CliOptions::default();
        assert_eq!(options.output_type, OutputType::ResponseBody);
        assert!(parse_option(&mut reader, &mut options).is_ok());
        assert_eq!(options.output_type, OutputType::NoOutput);
        assert_eq!(reader.cursor().pos, Pos::new(2, 1));
    }

    #[test]
    fn test_parse_option_no_assert() {
        let mut reader = Reader::new("--no-assert\n");
        let mut options = CliOptions::default();
        assert!(!options.no_assert);
        assert!(parse_option(&mut reader, &mut options).is_ok());
        assert!(options.no_assert);
        assert_eq!(reader.cursor().pos, Pos::new(2, 1));
    }

    #[test]
    fn test_parse_option_error() {
        let mut reader = Reader::new("verbose\n");
        let mut options = CliOptions::default();
        let err = parse_option(&mut reader, &mut options).unwrap_err();
        assert_eq!(err.pos, Pos::new(1, 1));
        assert_eq!(err.message, "Expecting an option starting with --");
    }

    #[test]
    fn test_parse_option_error_unknown() {
        let mut reader = Reader::new("--xxx");
        let mut options = CliOptions::default();
        let err = parse_option(&mut reader, &mut options).unwrap_err();
        assert_eq!(err.pos, Pos::new(1, 1));
        assert_eq!(err.message, "Unknown option <--xxx>");

        let mut reader = Reader::new("--verbosexxx");
        let mut options = CliOptions::default();
        let err = parse_option(&mut reader, &mut options).unwrap_err();
        assert_eq!(err.pos, Pos::new(1, 1));
        assert_eq!(err.message, "Unknown option <--verbosexxx>");
    }

    #[test]
    fn test_parse_option_error_missing_value() {
        let mut reader = Reader::new("--header\n");
        let mut options = CliOptions::default();
        let err = parse_option(&mut reader, &mut options).unwrap_err();
        assert_eq!(err.pos, Pos::new(1, 9));
        assert_eq!(
            err.message,
            "Expecting a value using space or '=' separator"
        );
    }

    #[test]
    fn test_parse_option_error_flag_with_value() {
        let mut reader = Reader::new("--verbose=1\n");
        let mut options = CliOptions::default();
        let err = parse_option(&mut reader, &mut options).unwrap_err();
        assert_eq!(err.pos, Pos::new(1, 10));
        assert_eq!(err.message, "Not expecting a value for this option");

        let mut reader = Reader::new("--max-redirs=a\n");
        let mut options = CliOptions::default();
        let err = parse_option(&mut reader, &mut options).unwrap_err();
        assert_eq!(err.pos, Pos::new(1, 14));
        assert_eq!(err.message, "Option --max-redirs requires an integer value");

        let mut reader = Reader::new("--delay=abc\n");
        let mut options = CliOptions::default();
        let err = parse_option(&mut reader, &mut options).unwrap_err();
        assert_eq!(err.pos, Pos::new(1, 9));
        assert_eq!(err.message, "Option --delay has an invalid duration");

        let mut reader = Reader::new("--limit-rate=abc\n");
        let mut options = CliOptions::default();
        let err = parse_option(&mut reader, &mut options).unwrap_err();
        assert_eq!(err.pos, Pos::new(1, 14));
        assert_eq!(err.message, "Option --limit-rate requires an integer value");
    }

    #[test]
    fn test_parse_config_error_trailing_chars_after_closing_quote() {
        let content = "--user-agent=\"Mozilla/5.0 A\" --verbose\n";
        let err = parse_config(content, CliOptions::default()).unwrap_err();
        assert_eq!(err.message, "characters after the closing quote");
    }
}
