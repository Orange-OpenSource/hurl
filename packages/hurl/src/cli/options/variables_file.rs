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
use crate::cli::options::variables::TypeKind;
use crate::cli::options::{variables, CliOptionsError};
use hurl::runner::Value;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::iter::Enumerate;
use std::path::{Path, PathBuf};

/// Represents a variables file, in the form of:
///
/// ```
/// var_1=foo
/// var_2=bar
/// var_3=baz
/// ```
///
/// [`VariablesFile`] is an iterator that returns a tuple ([`String`], [`Value`]) on each iteration.
pub struct VariablesFile {
    /// Iterator on this variables file lines.
    lines: Enumerate<Lines<BufReader<File>>>,
    /// Path of this variables file.
    path: PathBuf,
    /// How do we type variables?
    type_kind: TypeKind,
}

impl VariablesFile {
    /// Opens the variables file at `path`.
    /// Each variable will be typed: either variable type are inferred from their value, or variable
    /// are forced to be string.
    pub fn open(path: &Path, type_kind: TypeKind) -> Result<Self, CliOptionsError> {
        if !path.exists() {
            return Err(CliOptionsError::Error(format!(
                "Variables file {} does not exist",
                path.display()
            )));
        }

        let Ok(file) = File::open(path) else {
            let error = CliOptionsError::Error(format!("Error opening {}", path.display()));
            return Err(error);
        };
        let lines = BufReader::new(file).lines().enumerate();
        Ok(VariablesFile {
            lines,
            path: path.to_path_buf(),
            type_kind,
        })
    }
}

impl Iterator for VariablesFile {
    type Item = Result<(String, Value), CliOptionsError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (index, line) = self.lines.next()?;
            let line = match line {
                Ok(s) => s,
                Err(_) => {
                    let error = CliOptionsError::Error(format!(
                        "Can not parse line {} of {}",
                        index + 1,
                        self.path.display()
                    ));
                    return Some(Err(error));
                }
            };
            let line = line.trim();
            if line.starts_with('#') || line.is_empty() {
                continue;
            }
            let (name, value) = match variables::parse(line, self.type_kind) {
                Ok(v) => v,
                Err(err) => return Some(Err(err)),
            };
            return Some(Ok((name, value)));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cli::options::variables_file::{TypeKind, VariablesFile};
    use hurl::runner::{Number, Value};
    use std::path::PathBuf;
    use std::{env, fs};

    fn temp_file(name: &str) -> PathBuf {
        let dir = env::temp_dir();
        dir.join(name)
    }

    #[test]
    fn test_simple_properties_inferred() {
        let path = temp_file("file1.env");
        let content = r#"foo=bar
flag=true
id=123
"#;
        fs::write(&path, content).unwrap();
        let file = VariablesFile::open(&path, TypeKind::Inferred).unwrap();
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
    fn test_simple_properties_string() {
        let path = temp_file("file2.env");
        let content = r#"foo=bar
# With some comments
# bla bla bla
flag=true
id=123
"#;
        fs::write(&path, content).unwrap();
        let file = VariablesFile::open(&path, TypeKind::String).unwrap();
        let vars = file.collect::<Vec<_>>();
        assert_eq!(
            vars,
            vec![
                Ok(("foo".to_string(), Value::String("bar".to_string()))),
                Ok(("flag".to_string(), Value::String("true".to_string()))),
                Ok(("id".to_string(), Value::String("123".to_string()))),
            ]
        );
    }
}
