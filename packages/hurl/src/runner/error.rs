/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2024 Orange
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

use hurl_core::ast::SourceInfo;
use hurl_core::error::DisplaySourceError;
use hurl_core::text::{Style, StyledString};

use crate::http::HttpError;

/// Represents a single instance of a runtime error, usually triggered by running a
/// [`hurl_core::ast::Entry`]. Running a Hurl content (see [`crate::runner::run`]) returns a list of
/// result for each entry. Each entry result can contain a list of [`RunnerError`]. The runtime error variant
/// is defined in [`RunnerErrorKind`]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RunnerError {
    pub source_info: SourceInfo,
    pub kind: RunnerErrorKind,
    pub assert: bool,
}

impl RunnerError {
    pub fn new(source_info: SourceInfo, kind: RunnerErrorKind, assert: bool) -> RunnerError {
        RunnerError {
            source_info,
            kind,
            assert,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RunnerErrorKind {
    AssertBodyValueError {
        actual: String,
        expected: String,
    },
    AssertFailure {
        actual: String,
        expected: String,
        type_mismatch: bool,
    },
    AssertHeaderValueError {
        actual: String,
    },
    AssertStatus {
        actual: String,
    },
    AssertVersion {
        actual: String,
    },
    /// I/O read error on `path`.
    FileReadAccess {
        path: PathBuf,
    },
    /// I/O write error on `path`.
    FileWriteAccess {
        path: PathBuf,
        error: String,
    },
    FilterDecode(String),
    FilterInvalidEncoding(String),
    FilterInvalidInput(String),
    FilterMissingInput,
    Http(HttpError),
    InvalidJson {
        value: String,
    },
    InvalidRegex,
    NoQueryResult,
    QueryHeaderNotFound,
    QueryInvalidJsonpathExpression {
        value: String,
    },
    QueryInvalidXpathEval,
    QueryInvalidXml,
    QueryInvalidJson,
    TemplateVariableNotDefined {
        name: String,
    },
    TemplateVariableInvalidType {
        name: String,
        value: String,
        expecting: String,
    },
    UnrenderableVariable {
        name: String,
        value: String,
    },
    /// Unauthorized file access, check `--file-root` option.
    UnauthorizedFileAccess {
        path: PathBuf,
    },
}

/// Textual Output for runner errors
impl DisplaySourceError for RunnerError {
    fn source_info(&self) -> SourceInfo {
        self.source_info
    }

    fn description(&self) -> String {
        match &self.kind {
            RunnerErrorKind::AssertBodyValueError { .. } => "Assert body value".to_string(),
            RunnerErrorKind::AssertFailure { .. } => "Assert failure".to_string(),
            RunnerErrorKind::AssertHeaderValueError { .. } => "Assert header value".to_string(),
            RunnerErrorKind::AssertStatus { .. } => "Assert status code".to_string(),
            RunnerErrorKind::AssertVersion { .. } => "Assert HTTP version".to_string(),

            RunnerErrorKind::FileReadAccess { .. } => "File read access".to_string(),
            RunnerErrorKind::FileWriteAccess { .. } => "File write access".to_string(),
            RunnerErrorKind::FilterDecode { .. } => "Filter error".to_string(),
            RunnerErrorKind::FilterInvalidEncoding { .. } => "Filter error".to_string(),
            RunnerErrorKind::FilterInvalidInput { .. } => "Filter error".to_string(),
            RunnerErrorKind::FilterMissingInput => "Filter error".to_string(),
            RunnerErrorKind::Http(http_error) => http_error.description(),
            RunnerErrorKind::InvalidJson { .. } => "Invalid JSON".to_string(),
            RunnerErrorKind::InvalidRegex => "Invalid regex".to_string(),
            RunnerErrorKind::NoQueryResult => "No query result".to_string(),
            RunnerErrorKind::QueryHeaderNotFound => "Header not found".to_string(),
            RunnerErrorKind::QueryInvalidJson => "Invalid JSON".to_string(),
            RunnerErrorKind::QueryInvalidJsonpathExpression { .. } => {
                "Invalid JSONPath".to_string()
            }
            RunnerErrorKind::QueryInvalidXml => "Invalid XML".to_string(),
            RunnerErrorKind::QueryInvalidXpathEval => "Invalid XPath expression".to_string(),
            RunnerErrorKind::TemplateVariableInvalidType { .. } => {
                "Invalid variable type".to_string()
            }
            RunnerErrorKind::TemplateVariableNotDefined { .. } => "Undefined variable".to_string(),
            RunnerErrorKind::UnauthorizedFileAccess { .. } => {
                "Unauthorized file access".to_string()
            }
            RunnerErrorKind::UnrenderableVariable { .. } => "Unrenderable variable".to_string(),
        }
    }

    fn fixme(&self, content: &[&str]) -> StyledString {
        match &self.kind {
            RunnerErrorKind::AssertBodyValueError { actual, .. } => {
                let message = &format!("actual value is <{actual}>");
                let message = hurl_core::error::add_carets(message, self.source_info, content);
                color_red_multiline_string(&message)
            }
            RunnerErrorKind::AssertFailure {
                actual,
                expected,
                type_mismatch,
                ..
            } => {
                let additional = if *type_mismatch {
                    "\n   >>> types between actual and expected are not consistent"
                } else {
                    ""
                };
                let message = format!("   actual:   {actual}\n   expected: {expected}{additional}");
                color_red_multiline_string(&message)
            }
            RunnerErrorKind::AssertHeaderValueError { actual } => {
                let message = &format!("actual value is <{actual}>");
                let message = hurl_core::error::add_carets(message, self.source_info, content);
                color_red_multiline_string(&message)
            }
            RunnerErrorKind::AssertStatus { actual, .. } => {
                let message = &format!("actual value is <{actual}>");
                let message = hurl_core::error::add_carets(message, self.source_info, content);
                color_red_multiline_string(&message)
            }
            RunnerErrorKind::AssertVersion { actual, .. } => {
                let message = &format!("actual value is <{actual}>");
                let message = hurl_core::error::add_carets(message, self.source_info, content);
                color_red_multiline_string(&message)
            }

            RunnerErrorKind::FileReadAccess { path } => {
                let message = &format!("file {} can not be read", path.to_string_lossy());
                let message = hurl_core::error::add_carets(message, self.source_info, content);
                color_red_multiline_string(&message)
            }
            RunnerErrorKind::FileWriteAccess { path, error } => {
                let message = &format!("{} can not be written ({error})", path.to_string_lossy());
                let message = hurl_core::error::add_carets(message, self.source_info, content);
                color_red_multiline_string(&message)
            }
            RunnerErrorKind::FilterDecode(encoding) => {
                let message = &format!("value can not be decoded with <{encoding}> encoding");
                let message = hurl_core::error::add_carets(message, self.source_info, content);
                color_red_multiline_string(&message)
            }
            RunnerErrorKind::FilterInvalidEncoding(encoding) => {
                let message = &format!("<{encoding}> encoding is not supported");
                let message = hurl_core::error::add_carets(message, self.source_info, content);
                color_red_multiline_string(&message)
            }
            RunnerErrorKind::FilterInvalidInput(message) => {
                let message = &format!("invalid filter input: {message}");
                let message = hurl_core::error::add_carets(message, self.source_info, content);
                color_red_multiline_string(&message)
            }
            RunnerErrorKind::FilterMissingInput => {
                let message = "missing value to apply filter";
                let message = hurl_core::error::add_carets(message, self.source_info, content);
                color_red_multiline_string(&message)
            }
            RunnerErrorKind::Http(http_error) => {
                let message = http_error.message();
                let message = hurl_core::error::add_carets(&message, self.source_info, content);
                color_red_multiline_string(&message)
            }
            RunnerErrorKind::InvalidJson { value } => {
                let message = &format!("actual value is <{value}>");
                let message = hurl_core::error::add_carets(message, self.source_info, content);
                color_red_multiline_string(&message)
            }
            RunnerErrorKind::InvalidRegex => {
                let message = "regex expression is not valid";
                let message = hurl_core::error::add_carets(message, self.source_info, content);
                color_red_multiline_string(&message)
            }
            RunnerErrorKind::NoQueryResult => {
                let message = "The query didn't return any result";
                let message = hurl_core::error::add_carets(message, self.source_info, content);
                color_red_multiline_string(&message)
            }
            RunnerErrorKind::QueryHeaderNotFound => {
                let message = "this header has not been found in the response";
                let message = hurl_core::error::add_carets(message, self.source_info, content);
                color_red_multiline_string(&message)
            }
            RunnerErrorKind::QueryInvalidJson => {
                let message = "the HTTP response is not a valid JSON";
                let message = hurl_core::error::add_carets(message, self.source_info, content);
                color_red_multiline_string(&message)
            }
            RunnerErrorKind::QueryInvalidJsonpathExpression { value } => {
                let message = &format!("the JSONPath expression '{value}' is not valid");
                let message = hurl_core::error::add_carets(message, self.source_info, content);
                color_red_multiline_string(&message)
            }
            RunnerErrorKind::QueryInvalidXml => {
                let message = "the HTTP response is not a valid XML";
                let message = hurl_core::error::add_carets(message, self.source_info, content);
                color_red_multiline_string(&message)
            }
            RunnerErrorKind::QueryInvalidXpathEval => {
                let message = "the XPath expression is not valid";
                let message = hurl_core::error::add_carets(message, self.source_info, content);
                color_red_multiline_string(&message)
            }
            RunnerErrorKind::TemplateVariableInvalidType {
                value, expecting, ..
            } => {
                let message = &format!("expecting {expecting}, actual value is <{value}>");
                let message = hurl_core::error::add_carets(message, self.source_info, content);
                color_red_multiline_string(&message)
            }
            RunnerErrorKind::TemplateVariableNotDefined { name } => {
                let message = &format!("you must set the variable {name}");
                let message = hurl_core::error::add_carets(message, self.source_info, content);
                color_red_multiline_string(&message)
            }
            RunnerErrorKind::UnauthorizedFileAccess { path } => {
                let message = &format!(
                    "unauthorized access to file {}, check --file-root option",
                    path.to_string_lossy()
                );
                let message = hurl_core::error::add_carets(message, self.source_info, content);
                color_red_multiline_string(&message)
            }
            RunnerErrorKind::UnrenderableVariable { name, value } => {
                let message = &format!("variable <{name}> with value {value} can not be rendered");
                let message = hurl_core::error::add_carets(message, self.source_info, content);
                color_red_multiline_string(&message)
            }
        }
    }

    fn message(&self, content: &[&str]) -> StyledString {
        let mut text = StyledString::new();
        hurl_core::error::add_source_line(&mut text, content, self.source_info().start.line);
        text.append(self.fixme(content));
        text
    }
}

/// Color each line separately
fn color_red_multiline_string(s: &str) -> StyledString {
    let lines = s.split('\n');
    let mut s = StyledString::new();
    for (i, line) in lines.enumerate() {
        if i > 0 {
            s.push("\n");
        }
        s.push_with(line, Style::new().red().bold());
    }
    s
}

#[cfg(test)]
mod tests {
    use crate::http::HttpError;
    use crate::runner::{RunnerError, RunnerErrorKind};
    use hurl_core::ast::{Pos, SourceInfo};
    use hurl_core::error::{error_string, split_lines, DisplaySourceError};
    use hurl_core::text::Format;

    #[test]
    fn test_error_timeout() {
        let content = "GET http://unknown";
        let filename = "test.hurl";
        let kind = RunnerErrorKind::Http(HttpError::Libcurl {
            code: 6,
            description: "Could not resolve host: unknown".to_string(),
        });
        let error_source_info = SourceInfo::new(Pos::new(1, 5), Pos::new(1, 19));
        let entry_source_info = SourceInfo::new(Pos::new(1, 1), Pos::new(1, 19));
        let error = RunnerError::new(error_source_info, kind, true);
        assert_eq!(
            error
                .message(&split_lines(content))
                .to_string(Format::Plain),
            " GET http://unknown\n     ^^^^^^^^^^^^^^ (6) Could not resolve host: unknown"
        );
        assert_eq!(
            error_string(filename, content, &error, Some(entry_source_info), false),
            r#"HTTP connection
  --> test.hurl:1:5
   |
 1 | GET http://unknown
   |     ^^^^^^^^^^^^^^ (6) Could not resolve host: unknown
   |"#
        );
    }

    #[test]
    fn test_assert_error_status() {
        let content = r#"GET http://unknown
HTTP/1.0 200
"#;
        let filename = "test.hurl";
        let kind = RunnerErrorKind::AssertStatus {
            actual: "404".to_string(),
        };
        let error_source_info = SourceInfo::new(Pos::new(2, 10), Pos::new(2, 13));
        let entry_source_info = SourceInfo::new(Pos::new(1, 1), Pos::new(1, 18));
        let error = RunnerError::new(error_source_info, kind, true);

        assert_eq!(
            error
                .message(&split_lines(content))
                .to_string(Format::Plain),
            " HTTP/1.0 200\n          ^^^ actual value is <404>"
        );
        colored::control::set_override(true);
        assert_eq!(
            error.message(&split_lines(content)).to_string(Format::Ansi),
            " HTTP/1.0 200\n\u{1b}[1;31m          ^^^ actual value is <404>\u{1b}[0m",
        );

        assert_eq!(
            error_string(filename, content, &error, Some(entry_source_info), false),
            r#"Assert status code
  --> test.hurl:2:10
   |
   | GET http://unknown
 2 | HTTP/1.0 200
   |          ^^^ actual value is <404>
   |"#
        );
    }

    #[test]
    fn test_invalid_xpath_expression() {
        let content = r#"GET http://example.com
HTTP/1.0 200
[Asserts]
xpath "strong(//head/title)" == "Hello"
"#;
        let filename = "test.hurl";
        let error_source_info = SourceInfo::new(Pos::new(4, 7), Pos::new(4, 29));
        let entry_source_info = SourceInfo::new(Pos::new(1, 1), Pos::new(1, 22));
        let error = RunnerError::new(
            error_source_info,
            RunnerErrorKind::QueryInvalidXpathEval,
            true,
        );
        assert_eq!(
        &error.message( &split_lines(content)).to_string(Format::Plain),
        " xpath \"strong(//head/title)\" == \"Hello\"\n       ^^^^^^^^^^^^^^^^^^^^^^ the XPath expression is not valid"
    );
        assert_eq!(
            error_string(filename, content, &error, Some(entry_source_info), false),
            r#"Invalid XPath expression
  --> test.hurl:4:7
   |
   | GET http://example.com
   | ...
 4 | xpath "strong(//head/title)" == "Hello"
   |       ^^^^^^^^^^^^^^^^^^^^^^ the XPath expression is not valid
   |"#
        );
    }

    #[test]
    fn test_assert_error_jsonpath() {
        let content = r#"GET http://api
HTTP/1.0 200
[Asserts]
jsonpath "$.count" >= 5
"#;
        let filename = "test.hurl";
        let error_source_info = SourceInfo::new(Pos::new(4, 0), Pos::new(4, 0));
        let entry_source_info = SourceInfo::new(Pos::new(1, 1), Pos::new(1, 14));
        let error = RunnerError {
            source_info: error_source_info,
            kind: RunnerErrorKind::AssertFailure {
                actual: "int <2>".to_string(),
                expected: "greater than int <5>".to_string(),
                type_mismatch: false,
            },
            assert: true,
        };

        assert_eq!(
            error
                .message(&split_lines(content))
                .to_string(Format::Plain),
            r#" jsonpath "$.count" >= 5
   actual:   int <2>
   expected: greater than int <5>"#
        );

        assert_eq!(
            error_string(filename, content, &error, Some(entry_source_info), false),
            r#"Assert failure
  --> test.hurl:4:0
   |
   | GET http://api
   | ...
 4 | jsonpath "$.count" >= 5
   |   actual:   int <2>
   |   expected: greater than int <5>
   |"#
        );
    }

    #[test]
    fn test_assert_error_newline() {
        let content = r#"GET http://localhost
HTTP/1.0 200
```<p>Hello</p>
```
"#;
        let filename = "test.hurl";
        let kind = RunnerErrorKind::AssertBodyValueError {
            actual: "<p>Hello</p>\n\n".to_string(),
            expected: "<p>Hello</p>\n".to_string(),
        };
        let error_source_info = SourceInfo::new(Pos::new(3, 4), Pos::new(4, 1));
        let entry_source_info = SourceInfo::new(Pos::new(1, 1), Pos::new(1, 20));
        let error = RunnerError::new(error_source_info, kind, true);

        assert_eq!(
            error
                .message(&split_lines(content))
                .to_string(Format::Plain),
            " ```<p>Hello</p>\n    ^ actual value is <<p>Hello</p>\n\n      >"
        );
        assert_eq!(
            error_string(filename, content, &error, Some(entry_source_info), false),
            r#"Assert body value
  --> test.hurl:3:4
   |
   | GET http://localhost
   | ...
 3 | ```<p>Hello</p>
   |    ^ actual value is <<p>Hello</p>
   |
   |      >
   |"#
        );
    }
}
