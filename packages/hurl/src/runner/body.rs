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
use std::path::PathBuf;

use hurl_core::ast::{Base64, Body, Bytes, File, Hex, Template};

use crate::http;
use crate::runner::error::{RunnerError, RunnerErrorKind};
use crate::runner::json::eval_json_value;
use crate::runner::multiline::eval_multiline;
use crate::runner::template::eval_template;
use crate::runner::VariableSet;
use crate::util::path::ContextDir;

pub fn eval_body(
    body: &Body,
    variables: &VariableSet,
    context_dir: &ContextDir,
) -> Result<http::Body, RunnerError> {
    eval_bytes(&body.value, variables, context_dir)
}

pub fn eval_bytes(
    bytes: &Bytes,
    variables: &VariableSet,
    context_dir: &ContextDir,
) -> Result<http::Body, RunnerError> {
    match bytes {
        Bytes::OnelineString(value) => {
            let value = eval_template(value, variables)?;
            Ok(http::Body::Text(value))
        }
        Bytes::MultilineString(value) => {
            let value = eval_multiline(value, variables)?;
            Ok(http::Body::Text(value))
        }
        Bytes::Xml(value) => Ok(http::Body::Text(value.clone())),
        Bytes::Json(value) => {
            let value = eval_json_value(value, variables, true)?;
            Ok(http::Body::Text(value))
        }
        Bytes::Base64(Base64 { value, .. }) => Ok(http::Body::Binary(value.clone())),
        Bytes::Hex(Hex { value, .. }) => Ok(http::Body::Binary(value.clone())),
        Bytes::File(File { filename, .. }) => {
            let value = eval_file(filename, variables, context_dir)?;
            let filename = eval_template(filename, variables)?;
            Ok(http::Body::File(value, filename))
        }
    }
}

pub fn eval_file(
    filename: &Template,
    variables: &VariableSet,
    context_dir: &ContextDir,
) -> Result<Vec<u8>, RunnerError> {
    let file = eval_template(filename, variables)?;
    // In order not to leak any private date, we check that the user provided file
    // is a child of the context directory.
    let path = PathBuf::from(file);
    if !context_dir.is_access_allowed(&path) {
        let kind = RunnerErrorKind::UnauthorizedFileAccess { path };
        return Err(RunnerError::new(filename.source_info, kind, false));
    }
    let resolved_file = context_dir.resolved_path(&path);
    match std::fs::read(resolved_file) {
        Ok(value) => Ok(value),
        Err(_) => {
            let kind = RunnerErrorKind::FileReadAccess { path };
            Err(RunnerError::new(filename.source_info, kind, false))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use hurl_core::ast::{SourceInfo, TemplateElement, Whitespace};
    use hurl_core::reader::Pos;
    use hurl_core::typing::ToSource;

    use super::*;

    #[test]
    pub fn test_body_file() {
        // file, data.bin;
        let whitespace = Whitespace {
            value: String::from(" "),
            source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
        };

        let bytes = Bytes::File(File {
            space0: whitespace.clone(),
            filename: Template {
                delimiter: None,
                source_info: SourceInfo::new(Pos::new(1, 7), Pos::new(1, 15)),
                elements: vec![TemplateElement::String {
                    value: "tests/data.bin".to_string(),
                    source: "tests/data.bin".to_source(),
                }],
            },
            space1: whitespace,
        });

        let variables = VariableSet::new();
        let current_dir = std::env::current_dir().unwrap();
        let file_root = Path::new("");
        let context_dir = ContextDir::new(current_dir.as_path(), file_root);
        assert_eq!(
            eval_bytes(&bytes, &variables, &context_dir).unwrap(),
            http::Body::File(b"Hello World!".to_vec(), "tests/data.bin".to_string())
        );
    }

    #[test]
    pub fn test_body_file_error() {
        // file, data.bin;
        let whitespace = Whitespace {
            value: String::from(" "),
            source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
        };

        let bytes = Bytes::File(File {
            space0: whitespace.clone(),
            filename: Template {
                delimiter: None,
                source_info: SourceInfo::new(Pos::new(1, 7), Pos::new(1, 15)),
                elements: vec![TemplateElement::String {
                    value: "data.bin".to_string(),
                    source: "data.bin".to_source(),
                }],
            },
            space1: whitespace,
        });

        let variables = VariableSet::new();

        let current_dir = std::env::current_dir().unwrap();
        let file_root = Path::new("file_root");
        let context_dir = ContextDir::new(current_dir.as_path(), file_root);
        let error = eval_bytes(&bytes, &variables, &context_dir).err().unwrap();
        assert_eq!(
            error.kind,
            RunnerErrorKind::FileReadAccess {
                path: PathBuf::from("data.bin")
            }
        );
        assert_eq!(
            error.source_info,
            SourceInfo::new(Pos::new(1, 7), Pos::new(1, 15))
        );
    }
}
