/*
 * hurl (https://hurl.dev)
 * Copyright (C) 2020 Orange
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

pub fn eval_body(
    body: Body,
    variables: &HashMap<String, Value>,
    context_dir: String,
) -> Result<Vec<u8>, Error> {
    eval_bytes(body.value, variables, context_dir)
}

pub fn eval_bytes(
    bytes: Bytes,
    variables: &HashMap<String, Value>,
    context_dir: String,
) -> Result<Vec<u8>, Error> {
    match bytes {
        Bytes::RawString { value, .. } => {
            let value = eval_template(value, variables)?;
            Ok(value.into_bytes())
        }
        Bytes::Base64 { value, .. } => Ok(value),
        Bytes::Xml { value, .. } => Ok(value.into_bytes()),
        Bytes::Json { value, .. } => {
            let value = eval_json_value(value, variables)?;
            Ok(value.into_bytes())
        }
        Bytes::File { filename, .. } => {
            let path = Path::new(filename.value.as_str());
            let absolute_filename = if path.is_absolute() {
                filename.clone().value
            } else {
                Path::new(context_dir.as_str())
                    .join(filename.value)
                    .to_str()
                    .unwrap()
                    .to_string()
            };
            match std::fs::read(absolute_filename.clone()) {
                Ok(bytes) => Ok(bytes),
                Err(_) => Err(Error {
                    source_info: filename.source_info,
                    inner: RunnerError::FileReadAccess {
                        value: absolute_filename,
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

        let bytes = Bytes::File {
            space0: whitespace.clone(),
            filename: Filename {
                value: String::from("tests/data.bin"),
                source_info: SourceInfo::init(1, 7, 1, 15),
            },
            space1: whitespace.clone(),
        };

        let variables = HashMap::new();
        assert_eq!(
            eval_bytes(bytes, &variables, ".".to_string()).unwrap(),
            b"Hello World!"
        );
    }

    #[test]
    pub fn test_body_file_error() {
        // file, data.bin;
        let whitespace = Whitespace {
            value: String::from(" "),
            source_info: SourceInfo::init(0, 0, 0, 0),
        };

        let bytes = Bytes::File {
            space0: whitespace.clone(),
            filename: Filename {
                value: String::from("data.bin"),
                source_info: SourceInfo::init(1, 7, 1, 15),
            },
            space1: whitespace.clone(),
        };

        let variables = HashMap::new();

        let separator = if cfg!(windows) { "\\" } else { "/" };
        let error = eval_bytes(bytes, &variables, "current_dir".to_string())
            .err()
            .unwrap();
        assert_eq!(
            error.inner,
            RunnerError::FileReadAccess {
                value: String::from(format!("current_dir{}data.bin", separator))
            }
        );
        assert_eq!(error.source_info, SourceInfo::init(1, 7, 1, 15));
    }
}
