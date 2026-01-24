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
use crate::CliOptions;
use crate::CliOptionsError;
use crate::RunContext;
use crate::Verbosity;
use hurl_core::reader::Pos;
use hurl_core::reader::Reader;

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

/// Parse options
/// from config file if it is defined in `context`
/// using `default_options`
pub fn parse_config_file(
    context: &RunContext,
    default_options: CliOptions,
) -> Result<CliOptions, CliOptionsError> {
    if let Some(config_file_path) = context.config_file_path() {
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

/// Parse and process a single option from config file
/// Empty lines or lines starting with `#` are ignored
///
/// For the time-being only support the verbose option
/// TODO: Implement all the command-line options
fn parse_option(reader: &mut Reader, options: &mut CliOptions) -> Result<(), ConfigFileError> {
    // Skip space, empty lines and comments
    loop {
        reader.read_while(|c: char| c.is_whitespace());
        if reader.is_eof() {
            return Ok(());
        }
        if reader.peek() == Some('#') {
            reader.read_while(|c: char| c != '\n');
            if reader.peek() == Some('\n') {
                reader.read(); // consume newline
            }
        } else {
            break;
        }
    }

    // Parse option
    let save = reader.cursor();
    let option = reader.read_while(|c| c.is_alphabetic() || c == '-');
    if option == "--verbose" {
        options.verbosity = Some(Verbosity::Verbose);
        Ok(())
    } else {
        Err(ConfigFileError::new(
            save.pos,
            &format!("Unknown option <{}>", option),
        ))
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
        assert!(!options.color);
        assert_eq!(options.verbosity, Some(Verbosity::Verbose));
    }

    #[test]
    fn test_parse_option() {
        let mut reader = Reader::new("\n\n--verbose\n");
        let mut options = super::CliOptions::default();
        assert_eq!(reader.cursor().pos, Pos::new(1, 1));
        assert!(parse_option(&mut reader, &mut options).is_ok());
        assert_eq!(options.verbosity, Some(Verbosity::Verbose));
        assert_eq!(reader.cursor().pos, Pos::new(3, 10));
    }

    #[test]
    fn test_parse_no_option() {
        let mut reader = Reader::new("# ignore\n\n");
        let mut options = super::CliOptions::default();
        assert!(parse_option(&mut reader, &mut options).is_ok());
        assert_eq!(reader.cursor().pos, Pos::new(3, 1));
    }

    #[test]
    fn test_parse_invalid_option() {
        let mut reader = Reader::new("--xxx");
        let mut options = super::CliOptions::default();
        let err = parse_option(&mut reader, &mut options).unwrap_err();
        assert_eq!(err.pos, Pos::new(1, 1));
        assert_eq!(err.message, "Unknown option <--xxx>");

        let mut reader = Reader::new("--verbosexxx");
        let mut options = super::CliOptions::default();
        let err = parse_option(&mut reader, &mut options).unwrap_err();
        assert_eq!(err.pos, Pos::new(1, 1));
        assert_eq!(err.message, "Unknown option <--verbosexxx>");
    }
}
