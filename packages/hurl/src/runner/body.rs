/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2022 Orange
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
use std::path::Path;

use hurl_core::ast::*;

use super::core::{Error, RunnerError};
use super::json::eval_json_value;
use super::template::eval_template;
use super::value::Value;
use crate::http;
use crate::runner::path;

pub fn eval_body(
    body: Body,
    variables: &HashMap<String, Value>,
    context_dir: &Path,
) -> Result<http::Body, Error> {
    eval_bytes(body.value, variables, context_dir)
}

pub fn eval_bytes(
    bytes: Bytes,
    variables: &HashMap<String, Value>,
    context_dir: &Path,
) -> Result<http::Body, Error> {
    match bytes {
        // Body::Text
        Bytes::RawString(RawString { value, .. }) => {
            let value = eval_template(&value, variables)?;
            Ok(http::Body::Text(value))
        }
        Bytes::Xml { value, .. } => Ok(http::Body::Text(value)),
        Bytes::Json { value, .. } => {
            let value = eval_json_value(value, variables)?;
            Ok(http::Body::Text(value))
        }

        Bytes::Base64(Base64 { value, .. }) => Ok(http::Body::Binary(value)),
        Bytes::Hex(Hex { value, .. }) => Ok(http::Body::Binary(value)),
        Bytes::File(File { filename, .. }) => {
            let f = filename.value.as_str();
            let path = Path::new(f);
            let absolute_path = context_dir.join(path);
            // In order not to leak any private date, we check that the user provided file
            // is a child of the context directory.
            if !path::is_descendant(&absolute_path, context_dir) {
                return Err(Error {
                    source_info: filename.source_info,
                    inner: RunnerError::UnauthorizedFileAccess {
                        path: absolute_path,
                    },
                    assert: false,
                });
            }
            match std::fs::read(&absolute_path) {
                Ok(value) => Ok(http::Body::File(value, f.to_string())),
                Err(_) => Err(Error {
                    source_info: filename.source_info,
                    inner: RunnerError::FileReadAccess {
                        value: absolute_path.to_str().unwrap().to_string(),
                    },
                    assert: false,
                }),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use hurl_core::ast::SourceInfo;

    use super::*;

    #[test]
    pub fn test_body_file() {
        // file, data.bin;
        let whitespace = Whitespace {
            value: String::from(" "),
            source_info: SourceInfo::init(0, 0, 0, 0),
        };

        let bytes = Bytes::File(File {
            space0: whitespace.clone(),
            filename: Filename {
                value: String::from("tests/data.bin"),
                source_info: SourceInfo::init(1, 7, 1, 15),
            },
            space1: whitespace,
        });

        let variables = HashMap::new();
        assert_eq!(
            eval_bytes(bytes, &variables, Path::new("")).unwrap(),
            http::Body::File(b"Hello World!".to_vec(), "tests/data.bin".to_string())
        );
    }

    #[test]
    pub fn test_body_file_error() {
        // file, data.bin;
        let whitespace = Whitespace {
            value: String::from(" "),
            source_info: SourceInfo::init(0, 0, 0, 0),
        };

        let bytes = Bytes::File(File {
            space0: whitespace.clone(),
            filename: Filename {
                value: String::from("data.bin"),
                source_info: SourceInfo::init(1, 7, 1, 15),
            },
            space1: whitespace,
        });

        let variables = HashMap::new();

        let separator = if cfg!(windows) { "\\" } else { "/" };
        let error = eval_bytes(bytes, &variables, Path::new("current_dir"))
            .err()
            .unwrap();
        assert_eq!(
            error.inner,
            RunnerError::FileReadAccess {
                value: format!("current_dir{}data.bin", separator)
            }
        );
        assert_eq!(error.source_info, SourceInfo::init(1, 7, 1, 15));
    }
}
