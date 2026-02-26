use hurl_core::reader::CharPos;
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
use hurl_core::reader::Pos;
use hurl_core::reader::Reader;
use std::path::Path;

use super::{CliOptions, CliOptionsError, Verbosity};

#[derive(Debug)]
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
    if let Some(config_file_path) = config_file_path {
        if config_file_path.exists() {
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

fn skip_whitespace_and_comments(reader: &mut Reader) {
    loop {
        reader.read_while(|c: char| c.is_whitespace());
        if reader.is_eof() {
            break;
        }
        if reader.peek() == Some('#') {
            // Skip comment line
            reader.read_while(|c: char| c != '\n');
            if reader.peek() == Some('\n') {
                reader.read(); // consume newline
            }
        } else {
            break;
        }
    }
}
/// Parse and process a single option from config file
/// Empty lines or lines starting with `#` are ignored
fn parse_option(reader: &mut Reader, options: &mut CliOptions) -> Result<(), ConfigFileError> {
    skip_whitespace_and_comments(reader);
    if reader.is_eof() {
        return Ok(());
    }

    let save = reader.cursor();
    if reader.read_n(CharPos(2)) != "--" {
        return Err(ConfigFileError::new(
            save.pos,
            "Expecting an option starting with --",
        ));
    }

    let option_name = reader.read_while(|c: char| c.is_alphanumeric() || c == '-' || c == '_');
    match option_name.as_str() {
        "verbose" => {
            skip_whitespace_and_comments(reader);
            if reader.peek() == Some('=') {
                return Err(ConfigFileError::new(
                    save.pos,
                    "Option --verbose does not take a value",
                ));
            }
            options.verbosity = Some(Verbosity::Verbose);
            Ok(())
        }
        "header" => {
            skip_whitespace_and_comments(reader);
            let header_value = if reader.peek() == Some('=') {
                reader.read(); // consume '='
                reader.read_while(|c| c != '\n').trim().to_string()
            } else {
                if reader.is_eof() {
                    return Err(ConfigFileError::new(
                        save.pos,
                        "Option --header requires a value",
                    ));
                }

                reader.read_while(|c| c != '\n').trim().to_string()
            };
            if header_value.is_empty() {
                return Err(ConfigFileError::new(
                    save.pos,
                    "Option --header requires a value",
                ));
            }
            options.headers.push(header_value);

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

        let mut reader = Reader::new("--header\nheader2:value2\n--verbose\n");
        let mut options = CliOptions::default();
        assert!(parse_option(&mut reader, &mut options).is_ok());
        assert_eq!(options.headers, vec!["header2:value2"]);
        assert_eq!(reader.cursor().pos, Pos::new(2, 15));

        let mut reader = Reader::new("--header\n--test:1\n");
        let mut options = CliOptions::default();
        assert!(parse_option(&mut reader, &mut options).is_ok());
        assert_eq!(options.headers, vec!["--test:1"]);
        assert_eq!(reader.cursor().pos, Pos::new(2, 9));
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
        assert_eq!(err.pos, Pos::new(1, 1));
        assert_eq!(err.message, "Option --header requires a value");
    }

    #[test]
    fn test_parse_option_error_flag_with_value() {
        let mut reader = Reader::new("--verbose=1\n");
        let mut options = CliOptions::default();
        let err = parse_option(&mut reader, &mut options).unwrap_err();
        assert_eq!(err.pos, Pos::new(1, 1));
        assert_eq!(err.message, "Option --verbose does not take a value");
    }
}
