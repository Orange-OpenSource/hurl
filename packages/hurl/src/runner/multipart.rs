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
use std::ffi::OsStr;
use std::path::Path;

use hurl_core::ast::{FileParam, FileValue, KeyValue, MultipartParam};

use crate::http;
use crate::runner::body::eval_file;
use crate::runner::error::RunnerError;
use crate::runner::template::eval_template;
use crate::runner::VariableSet;
use crate::util::path::ContextDir;

pub fn eval_multipart_param(
    multipart_param: &MultipartParam,
    variables: &VariableSet,
    context_dir: &ContextDir,
) -> Result<http::MultipartParam, RunnerError> {
    match multipart_param {
        MultipartParam::Param(KeyValue { key, value, .. }) => {
            let name = eval_template(key, variables)?;
            let value = eval_template(value, variables)?;
            Ok(http::MultipartParam::Param(http::Param { name, value }))
        }
        MultipartParam::FileParam(param) => {
            let file_param = eval_file_param(param, context_dir, variables)?;
            Ok(http::MultipartParam::FileParam(file_param))
        }
    }
}

pub fn eval_file_param(
    file_param: &FileParam,
    context_dir: &ContextDir,
    variables: &VariableSet,
) -> Result<http::FileParam, RunnerError> {
    let name = eval_template(&file_param.key, variables)?;
    let filename = eval_template(&file_param.value.filename, variables)?;
    let data = eval_file(&file_param.value.filename, variables, context_dir)?;
    let content_type = file_value_content_type(&file_param.value, variables)?;
    Ok(http::FileParam {
        name,
        filename,
        data,
        content_type,
    })
}

pub fn file_value_content_type(
    file_value: &FileValue,
    variables: &VariableSet,
) -> Result<String, RunnerError> {
    let value = match file_value.content_type.clone() {
        None => {
            let value = eval_template(&file_value.filename, variables)?;
            match Path::new(value.as_str())
                .extension()
                .and_then(OsStr::to_str)
            {
                Some("gif") => "image/gif".to_string(),
                Some("jpg") => "image/jpeg".to_string(),
                Some("jpeg") => "image/jpeg".to_string(),
                Some("png") => "image/png".to_string(),
                Some("svg") => "image/svg+xml".to_string(),
                Some("txt") => "text/plain".to_string(),
                Some("htm") => "text/html".to_string(),
                Some("html") => "text/html".to_string(),
                Some("pdf") => "application/pdf".to_string(),
                Some("xml") => "application/xml".to_string(),
                _ => "application/octet-stream".to_string(),
            }
        }
        Some(content_type) => content_type,
    };
    Ok(value)
}

#[cfg(test)]
mod tests {
    use hurl_core::ast::{LineTerminator, SourceInfo, Template, TemplateElement, Whitespace};
    use hurl_core::reader::Pos;
    use hurl_core::typing::ToSource;

    use super::*;

    pub fn whitespace() -> Whitespace {
        Whitespace {
            value: String::from(" "),
            source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
        }
    }

    #[test]
    pub fn test_eval_file_param() {
        let line_terminator = LineTerminator {
            space0: whitespace(),
            comment: None,
            newline: whitespace(),
        };
        let current_dir = std::env::current_dir().unwrap();
        let file_root = Path::new("tests");
        let context_dir = ContextDir::new(current_dir.as_path(), file_root);
        let variables = VariableSet::default();
        let param = eval_file_param(
            &FileParam {
                line_terminators: vec![],
                space0: whitespace(),
                key: Template {
                    delimiter: None,
                    elements: vec![TemplateElement::String {
                        value: "upload1".to_string(),
                        source: "upload1".to_source(),
                    }],
                    source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                },
                space1: whitespace(),
                space2: whitespace(),
                value: FileValue {
                    space0: whitespace(),
                    filename: Template {
                        delimiter: None,
                        source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                        elements: vec![TemplateElement::String {
                            value: "hello.txt".to_string(),
                            source: "hello.txt".to_source(),
                        }],
                    },
                    space1: whitespace(),
                    space2: whitespace(),
                    content_type: None,
                },
                line_terminator0: line_terminator,
            },
            &context_dir,
            &variables,
        )
        .unwrap();
        assert_eq!(
            param,
            http::FileParam {
                name: "upload1".to_string(),
                filename: "hello.txt".to_string(),
                data: b"Hello World!".to_vec(),
                content_type: "text/plain".to_string(),
            }
        );
    }

    #[test]
    pub fn test_file_value_content_type() {
        let variables = VariableSet::default();
        assert_eq!(
            file_value_content_type(
                &FileValue {
                    space0: whitespace(),
                    filename: Template {
                        delimiter: None,
                        source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                        elements: vec![TemplateElement::String {
                            value: "hello.txt".to_string(),
                            source: "hello.txt".to_source()
                        }],
                    },
                    space1: whitespace(),
                    space2: whitespace(),
                    content_type: None,
                },
                &variables
            )
            .unwrap(),
            "text/plain".to_string()
        );

        assert_eq!(
            file_value_content_type(
                &FileValue {
                    space0: whitespace(),
                    filename: Template {
                        delimiter: None,
                        source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                        elements: vec![TemplateElement::String {
                            value: "hello.html".to_string(),
                            source: "hello.html".to_source()
                        }],
                    },
                    space1: whitespace(),
                    space2: whitespace(),
                    content_type: None,
                },
                &variables
            )
            .unwrap(),
            "text/html".to_string()
        );

        assert_eq!(
            file_value_content_type(
                &FileValue {
                    space0: whitespace(),
                    filename: Template {
                        delimiter: None,
                        source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                        elements: vec![TemplateElement::String {
                            value: "hello.txt".to_string(),
                            source: "hello.txt".to_source()
                        }],
                    },
                    space1: whitespace(),
                    space2: whitespace(),
                    content_type: Some("text/html".to_string()),
                },
                &variables
            )
            .unwrap(),
            "text/html".to_string()
        );

        assert_eq!(
            file_value_content_type(
                &FileValue {
                    space0: whitespace(),
                    filename: Template {
                        delimiter: None,
                        source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                        elements: vec![TemplateElement::String {
                            value: "hello".to_string(),
                            source: "hello".to_source()
                        }],
                    },
                    space1: whitespace(),
                    space2: whitespace(),
                    content_type: None,
                },
                &variables
            )
            .unwrap(),
            "application/octet-stream".to_string()
        );
    }
}
