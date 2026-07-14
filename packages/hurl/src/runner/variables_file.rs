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
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::{Path, PathBuf};

use hurl_core::ast::{SourceInfo, is_variable_reserved};

use super::error::{RunnerError, RunnerErrorKind};
use super::number::Number;
use super::value::Value;

/// Represents a variables file, in the form of:
///
/// ```text
/// var_1=foo
/// var_2=bar
/// var_3=baz
/// ```
///
/// [`VariablesFile`] is an iterator that returns a tuple ([`String`], [`Value`]) on each iteration.
pub struct VariablesFile {
    /// Iterator on this variables file lines.
    lines: Lines<BufReader<File>>,
    /// Path of this variables file.
    path: PathBuf,
    /// Source information of this variables file option.
    source_info: SourceInfo,
}

impl VariablesFile {
    /// Opens the variables file at `path`.
    pub fn open(path: &Path, source_info: SourceInfo) -> Result<Self, RunnerError> {
        let file = File::open(path).map_err(|_| {
            let kind = RunnerErrorKind::FileReadAccess {
                path: path.to_path_buf(),
            };
            RunnerError::new(source_info, kind, false)
        })?;
        let lines = BufReader::new(file).lines();
        Ok(VariablesFile {
            lines,
            path: path.to_path_buf(),
            source_info,
        })
    }
}

impl Iterator for VariablesFile {
    type Item = Result<(String, Value), RunnerError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let line = self.lines.next()?;
            let line = match line {
                Ok(s) => s,
                Err(_) => {
                    let kind = RunnerErrorKind::FileReadAccess {
                        path: self.path.clone(),
                    };
                    let error = RunnerError::new(self.source_info, kind, false);
                    return Some(Err(error));
                }
            };
            let line = line.trim();
            if line.starts_with('#') || line.is_empty() {
                continue;
            }
            let (name, value) = match parse(line, self.source_info) {
                Ok(v) => v,
                Err(err) => return Some(Err(err)),
            };
            return Some(Ok((name, value)));
        }
    }
}

/// Parses a string "name=value" as a pair of `String` and `Value`.
fn parse(s: &str, source_info: SourceInfo) -> Result<(String, Value), RunnerError> {
    match s.find('=') {
        None => Err(invalid_option_value(
            s,
            format!("Missing value for variable {s}!"),
            source_info,
        )),
        Some(index) => {
            let (name, value) = s.split_at(index);
            if is_variable_reserved(name) {
                return Err(invalid_option_value(
                    s,
                    format!(
                        "Variable {name} conflicts with the {name} function, use a different name."
                    ),
                    source_info,
                ));
            }
            let value = parse_value(s, &value[1..], source_info)?;
            Ok((name.to_string(), value))
        }
    }
}

/// Parses a `value` as a [`Value`].
fn parse_value(line: &str, s: &str, source_info: SourceInfo) -> Result<Value, RunnerError> {
    if s == "true" {
        Ok(Value::Bool(true))
    } else if s == "false" {
        Ok(Value::Bool(false))
    } else if s == "null" {
        Ok(Value::Null)
    } else if let Ok(v) = s.parse::<i64>() {
        Ok(Value::Number(Number::Integer(v)))
    } else if s.chars().all(char::is_numeric) {
        Ok(Value::Number(Number::BigInteger(s.to_string())))
    } else if let Ok(v) = s.parse::<f64>() {
        Ok(Value::Number(Number::Float(v)))
    } else if let Some(s) = s.strip_prefix('"') {
        if let Some(s) = s.strip_suffix('"') {
            Ok(Value::String(s.to_string()))
        } else {
            Err(invalid_option_value(
                line,
                "Value should end with a double quote".to_string(),
                source_info,
            ))
        }
    } else {
        Ok(Value::String(s.to_string()))
    }
}

fn invalid_option_value(value: &str, message: String, source_info: SourceInfo) -> RunnerError {
    let kind = RunnerErrorKind::InvalidOptionValue {
        name: "variables-file".to_string(),
        value: value.to_string(),
        message,
    };
    RunnerError::new(source_info, kind, false)
}

#[cfg(test)]
mod tests {
    use hurl_core::ast::SourceInfo;
    use hurl_core::reader::Pos;

    use super::*;

    use std::path::PathBuf;
    use std::{env, fs};

    fn temp_file(name: &str) -> PathBuf {
        let dir = env::temp_dir();
        dir.join(name)
    }

    fn source_info() -> SourceInfo {
        SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0))
    }

    #[test]
    fn test_simple_properties_inferred() {
        let path = temp_file("runner-variables-file.env");
        let content = r#"foo=bar

# With some comments
# bla bla bla
flag=true
id=123
"#;
        fs::write(&path, content).unwrap();
        let file = VariablesFile::open(&path, source_info()).unwrap();
        let vars = file.collect::<Vec<_>>();
        assert_eq!(
            vars,
            vec![
                Ok(("foo".to_string(), Value::String("bar".to_string()))),
                Ok(("flag".to_string(), Value::Bool(true))),
                Ok(("id".to_string(), Value::Number(Number::Integer(123)))),
            ]
        );
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            parse("name=Jennifer", source_info()).unwrap(),
            ("name".to_string(), Value::String("Jennifer".to_string()))
        );
        assert_eq!(
            parse("female=true", source_info()).unwrap(),
            ("female".to_string(), Value::Bool(true))
        );
        assert_eq!(
            parse("age=30", source_info()).unwrap(),
            ("age".to_string(), Value::Number(Number::Integer(30)))
        );
        assert_eq!(
            parse("height=1.7", source_info()).unwrap(),
            ("height".to_string(), Value::Number(Number::Float(1.7)))
        );
        assert_eq!(
            parse("id=\"123\"", source_info()).unwrap(),
            ("id".to_string(), Value::String("123".to_string()))
        );
        assert_eq!(
            parse("id=9223372036854775808", source_info()).unwrap(),
            (
                "id".to_string(),
                Value::Number(Number::BigInteger("9223372036854775808".to_string()))
            )
        );
        assert_eq!(
            parse("a_null=null", source_info()).unwrap(),
            ("a_null".to_string(), Value::Null)
        );
    }

    #[test]
    fn test_parse_error() {
        assert_eq!(
            parse("name", source_info()).err().unwrap().kind,
            RunnerErrorKind::InvalidOptionValue {
                name: "variables-file".to_string(),
                value: "name".to_string(),
                message: "Missing value for variable name!".to_string(),
            }
        );
        assert_eq!(
            parse("newUuid=value", source_info()).err().unwrap().kind,
            RunnerErrorKind::InvalidOptionValue {
                name: "variables-file".to_string(),
                value: "newUuid=value".to_string(),
                message:
                    "Variable newUuid conflicts with the newUuid function, use a different name."
                        .to_string(),
            }
        );
    }

    #[test]
    fn test_parse_value() {
        assert_eq!(
            parse_value("Jennifer", "Jennifer", source_info()).unwrap(),
            Value::String("Jennifer".to_string())
        );
        assert_eq!(
            parse_value("true", "true", source_info()).unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            parse_value("30", "30", source_info()).unwrap(),
            Value::Number(Number::Integer(30))
        );
        assert_eq!(
            parse_value("1.7", "1.7", source_info()).unwrap(),
            Value::Number(Number::Float(1.7))
        );
        assert_eq!(
            parse_value("1.0", "1.0", source_info()).unwrap(),
            Value::Number(Number::Float(1.0))
        );
        assert_eq!(
            parse_value("-1.0", "-1.0", source_info()).unwrap(),
            Value::Number(Number::Float(-1.0))
        );
        assert_eq!(
            parse_value("\"123\"", "\"123\"", source_info()).unwrap(),
            Value::String("123".to_string())
        );
        assert_eq!(
            parse_value("null", "null", source_info()).unwrap(),
            Value::Null
        );
    }

    #[test]
    fn test_parse_value_error() {
        assert_eq!(
            parse_value("\"123", "\"123", source_info())
                .err()
                .unwrap()
                .kind,
            RunnerErrorKind::InvalidOptionValue {
                name: "variables-file".to_string(),
                value: "\"123".to_string(),
                message: "Value should end with a double quote".to_string(),
            }
        );
    }
}
