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
use std::cmp;

use crate::ast::SourceInfo;
use crate::error;
use crate::error::DisplaySourceError;
use crate::reader::Pos;
use crate::text::{Style, StyledString};

/// Represents a parser error.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseError {
    pub pos: Pos,
    pub recoverable: bool,
    pub kind: ParseErrorKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseErrorKind {
    DuplicateSection,
    EscapeChar,
    Expecting { value: String },
    FileContentType,
    Filename,
    GraphQlVariables,
    HexDigit,
    InvalidCookieAttribute,
    InvalidDurationUnit(String),
    InvalidOption(String),
    Json(JsonErrorVariant),
    JsonPathExpr,
    Method { name: String },
    Multiline,
    MultilineAttribute(String),
    OddNumberOfHexDigits,
    Predicate,
    PredicateValue,
    RegexExpr { message: String },
    RequestSection,
    RequestSectionName { name: String },
    ResponseSection,
    ResponseSectionName { name: String },
    Space,
    Status,
    TemplateVariable,
    Unicode,
    UrlIllegalCharacter(char),
    UrlInvalidStart,
    Variable(String),
    Version,
    XPathExpr,
    Xml,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JsonErrorVariant {
    TrailingComma,
    ExpectingElement,
    EmptyElement,
}

impl ParseError {
    /// Creates a new error for the position `pos`, of type `inner`.
    pub fn new(pos: Pos, recoverable: bool, kind: ParseErrorKind) -> ParseError {
        ParseError {
            pos,
            recoverable,
            kind,
        }
    }
}

impl DisplaySourceError for ParseError {
    fn source_info(&self) -> SourceInfo {
        SourceInfo {
            start: self.pos,
            end: self.pos,
        }
    }

    fn description(&self) -> String {
        match self.kind {
            ParseErrorKind::DuplicateSection => "Parsing section".to_string(),
            ParseErrorKind::EscapeChar => "Parsing escape character".to_string(),
            ParseErrorKind::Expecting { .. } => "Parsing literal".to_string(),
            ParseErrorKind::FileContentType => "Parsing file content type".to_string(),
            ParseErrorKind::Filename => "Parsing filename".to_string(),
            ParseErrorKind::GraphQlVariables => "Parsing GraphQL variables".to_string(),
            ParseErrorKind::HexDigit => "Parsing hexadecimal number".to_string(),
            ParseErrorKind::InvalidCookieAttribute => "Parsing cookie attribute".to_string(),
            ParseErrorKind::InvalidOption(_) => "Parsing option".to_string(),
            ParseErrorKind::InvalidDurationUnit(_) => "Parsing duration".to_string(),
            ParseErrorKind::Json(_) => "Parsing JSON".to_string(),
            ParseErrorKind::JsonPathExpr => "Parsing JSONPath expression".to_string(),
            ParseErrorKind::Method { .. } => "Parsing method".to_string(),
            ParseErrorKind::Multiline => "Parsing multiline".to_string(),
            ParseErrorKind::MultilineAttribute(..) => "Parsing multiline".to_string(),
            ParseErrorKind::OddNumberOfHexDigits => "Parsing hex bytearray".to_string(),
            ParseErrorKind::Predicate => "Parsing predicate".to_string(),
            ParseErrorKind::PredicateValue => "Parsing predicate value".to_string(),
            ParseErrorKind::RegexExpr { .. } => "Parsing regex".to_string(),
            ParseErrorKind::RequestSection => "Parsing section".to_string(),
            ParseErrorKind::RequestSectionName { .. } => "Parsing request section name".to_string(),
            ParseErrorKind::ResponseSection => "Parsing section".to_string(),
            ParseErrorKind::ResponseSectionName { .. } => {
                "Parsing response section name".to_string()
            }
            ParseErrorKind::Space => "Parsing space".to_string(),
            ParseErrorKind::Status => "Parsing status code".to_string(),
            ParseErrorKind::TemplateVariable => "Parsing template variable".to_string(),
            ParseErrorKind::Unicode => "Parsing unicode literal".to_string(),
            ParseErrorKind::UrlIllegalCharacter(_) => "Parsing URL".to_string(),
            ParseErrorKind::UrlInvalidStart => "Parsing URL".to_string(),
            ParseErrorKind::Variable(_) => "Parsing variable".to_string(),
            ParseErrorKind::Version => "Parsing version".to_string(),
            ParseErrorKind::XPathExpr => "Parsing XPath expression".to_string(),
            ParseErrorKind::Xml => "Parsing XML".to_string(),
        }
    }

    fn fixme(&self, content: &[&str]) -> StyledString {
        let message = match &self.kind {
            ParseErrorKind::DuplicateSection => "the section is already defined".to_string(),
            ParseErrorKind::EscapeChar => "the escaping sequence is not valid".to_string(),
            ParseErrorKind::Expecting { value } => format!("expecting '{value}'"),
            ParseErrorKind::FileContentType => "expecting a content type".to_string(),
            ParseErrorKind::Filename => "expecting a filename".to_string(),
            ParseErrorKind::GraphQlVariables => {
                "GraphQL variables is not a valid JSON object".to_string()
            }
            ParseErrorKind::HexDigit => "expecting a valid hexadecimal number".to_string(),
            ParseErrorKind::InvalidCookieAttribute => {
                "the cookie attribute is not valid".to_string()
            }
            ParseErrorKind::InvalidDurationUnit(name) => {
                let valid_values = ["ms", "s"];
                let default = format!("Valid values are {}", valid_values.join(", "));
                let did_you_mean = did_you_mean(&valid_values, name.as_str(), &default);
                format!("the duration unit is not valid. {did_you_mean}")
            }
            ParseErrorKind::InvalidOption(name) => {
                let valid_values = [
                    "aws-sigv4",
                    "cacert",
                    "cert",
                    "compressed",
                    "connect-to",
                    "delay",
                    "insecure",
                    "header",
                    "http1.0",
                    "http1.1",
                    "http2",
                    "http3",
                    "ipv4",
                    "ipv6",
                    "key",
                    "location",
                    "max-redirs",
                    "output",
                    "path-as-is",
                    "proxy",
                    "redirects",
                    "resolve",
                    "retry",
                    "retry-interval",
                    "skip",
                    "unix-socket",
                    "variable",
                    "verbose",
                    "very-verbose",
                ];
                let default = format!("Valid values are {}", valid_values.join(", "));
                let did_you_mean = did_you_mean(&valid_values, name.as_str(), &default);
                format!("the option name is not valid. {did_you_mean}")
            }
            ParseErrorKind::Json(variant) => match variant {
                JsonErrorVariant::TrailingComma => "trailing comma is not allowed".to_string(),
                JsonErrorVariant::EmptyElement => {
                    "expecting an element; found empty element instead".to_string()
                }
                JsonErrorVariant::ExpectingElement => {
                    "expecting a boolean, number, string, array, object or null".to_string()
                }
            },
            ParseErrorKind::JsonPathExpr => "expecting a JSONPath expression".to_string(),
            ParseErrorKind::Method { name } => {
                let valid_values = [
                    "GET", "HEAD", "POST", "PUT", "DELETE", "CONNECT", "OPTIONS", "TRACE", "PATCH",
                ];
                let default = format!("Valid values are {}", valid_values.join(", "));
                let did_you_mean = did_you_mean(&valid_values, name.as_str(), &default);
                format!("the HTTP method <{name}> is not valid. {did_you_mean}")
            }
            ParseErrorKind::Multiline => "the multiline is not valid".to_string(),
            ParseErrorKind::MultilineAttribute(name) => format!("Invalid attribute {name}"),
            ParseErrorKind::OddNumberOfHexDigits => {
                "expecting an even number of hex digits".to_string()
            }
            ParseErrorKind::Predicate => "expecting a predicate".to_string(),
            ParseErrorKind::PredicateValue => "invalid predicate value".to_string(),
            ParseErrorKind::RegexExpr { message } => format!("invalid Regex expression: {message}"),
            ParseErrorKind::RequestSection => {
                "this is not a valid section for a request".to_string()
            }
            ParseErrorKind::RequestSectionName { name } => {
                let valid_values = [
                    "QueryStringParams",
                    "FormParams",
                    "MultipartFormData",
                    "Cookies",
                    "Options",
                ];
                let default = format!("Valid values are {}", valid_values.join(", "));
                let did_you_mean = did_you_mean(&valid_values, name.as_str(), &default);
                format!("the section is not valid. {did_you_mean}")
            }
            ParseErrorKind::ResponseSection => {
                "this is not a valid section for a response".to_string()
            }
            ParseErrorKind::ResponseSectionName { name } => {
                let valid_values = ["Captures", "Asserts"];
                let default = "Valid values are Captures or Asserts";
                let did_your_mean = did_you_mean(&valid_values, name.as_str(), default);
                format!("the section is not valid. {did_your_mean}")
            }
            ParseErrorKind::Space => "expecting a space".to_string(),
            ParseErrorKind::Status => "HTTP status code is not valid".to_string(),
            ParseErrorKind::TemplateVariable => "expecting a variable".to_string(),
            ParseErrorKind::Unicode => "Invalid unicode literal".to_string(),
            ParseErrorKind::UrlIllegalCharacter(c) => format!("illegal character <{c}>"),
            ParseErrorKind::UrlInvalidStart => "expecting http://, https:// or {{".to_string(),
            ParseErrorKind::Variable(message) => message.clone(),
            ParseErrorKind::Version => {
                "HTTP version must be HTTP, HTTP/1.0, HTTP/1.1 or HTTP/2".to_string()
            }
            ParseErrorKind::XPathExpr => "expecting a XPath expression".to_string(),
            ParseErrorKind::Xml => "invalid XML".to_string(),
        };

        let message = error::add_carets(&message, self.source_info(), content);
        let mut s = StyledString::new();
        s.push_with(&message, Style::new().red().bold());
        s
    }
}

impl crate::combinator::ParseError for ParseError {
    fn is_recoverable(&self) -> bool {
        self.recoverable
    }

    fn to_recoverable(self) -> Self {
        ParseError {
            recoverable: true,
            ..self
        }
    }

    fn to_non_recoverable(self) -> Self {
        ParseError {
            recoverable: false,
            ..self
        }
    }
}

fn did_you_mean(valid_values: &[&str], actual: &str, default: &str) -> String {
    if let Some(suggest) = suggestion(valid_values, actual) {
        format!("Did you mean {suggest}?")
    } else {
        default.to_string()
    }
}

fn suggestion(valid_values: &[&str], actual: &str) -> Option<String> {
    for value in valid_values {
        if levenshtein_distance(
            value.to_lowercase().as_str(),
            actual.to_lowercase().as_str(),
        ) < 2
        {
            return Some(value.to_string());
        }
    }
    None
}

// From https://en.wikibooks.org/wiki/Algorithm_Implementation/Strings/Levenshtein_distance#Rust
fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let v1: Vec<char> = s1.chars().collect();
    let v2: Vec<char> = s2.chars().collect();

    fn min3<T: Ord>(v1: T, v2: T, v3: T) -> T {
        cmp::min(v1, cmp::min(v2, v3))
    }
    fn delta(x: char, y: char) -> usize {
        usize::from(x != y)
    }

    let mut column: Vec<usize> = (0..=v1.len()).collect();
    for x in 1..=v2.len() {
        column[0] = x;
        let mut lastdiag = x - 1;
        for y in 1..=v1.len() {
            let olddiag = column[y];
            column[y] = min3(
                column[y] + 1,
                column[y - 1] + 1,
                lastdiag + delta(v1[y - 1], v2[x - 1]),
            );
            lastdiag = olddiag;
        }
    }
    column[v1.len()]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::OutputFormat;

    #[test]
    fn test_levenshtein() {
        assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
        assert_eq!(levenshtein_distance("Saturday", "Sunday"), 3);
    }

    #[test]
    fn test_suggestion() {
        let valid_values = ["Captures", "Asserts"];
        assert_eq!(
            suggestion(&valid_values, "Asserts"),
            Some("Asserts".to_string())
        );
        assert_eq!(
            suggestion(&valid_values, "Assert"),
            Some("Asserts".to_string())
        );
        assert_eq!(
            suggestion(&valid_values, "assert"),
            Some("Asserts".to_string())
        );
        assert_eq!(suggestion(&valid_values, "asser"), None);
    }

    #[test]
    fn test_parsing_error() {
        let content = "GET abc";
        let filename = "test.hurl";
        let error = ParseError {
            pos: Pos::new(1, 5),
            recoverable: false,
            kind: ParseErrorKind::UrlInvalidStart,
        };
        assert_eq!(
            error.to_string(filename, content, None, OutputFormat::Terminal(false)),
            r#"Parsing URL
  --> test.hurl:1:5
   |
 1 | GET abc
   |     ^ expecting http://, https:// or {{
   |"#
        );
    }
}
