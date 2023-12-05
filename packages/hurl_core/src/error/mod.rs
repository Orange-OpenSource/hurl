/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2023 Orange
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
use core::cmp;

use crate::ast::SourceInfo;
use crate::parser::{self, JsonErrorVariant, ParseError};

pub trait Error {
    fn source_info(&self) -> SourceInfo;
    fn description(&self) -> String;
    fn fixme(&self) -> String;
}

impl Error for parser::Error {
    fn source_info(&self) -> SourceInfo {
        SourceInfo {
            start: self.pos,
            end: self.pos,
        }
    }

    fn description(&self) -> String {
        match self.inner {
            ParseError::DuplicateSection => "Parsing section".to_string(),
            ParseError::EscapeChar => "Parsing escape character".to_string(),
            ParseError::Expecting { .. } => "Parsing literal".to_string(),
            ParseError::FileContentType => "Parsing file content type".to_string(),
            ParseError::Filename => "Parsing filename".to_string(),
            ParseError::GraphQlVariables => "Parsing GraphQL variables".to_string(),
            ParseError::HexDigit => "Parsing hexadecimal number".to_string(),
            ParseError::InvalidCookieAttribute => "Parsing cookie attribute".to_string(),
            ParseError::InvalidOption(_) => "Parsing option".to_string(),
            ParseError::Json(_) => "Parsing JSON".to_string(),
            ParseError::JsonPathExpr => "Parsing JSONPath expression".to_string(),
            ParseError::Method { .. } => "Parsing method".to_string(),
            ParseError::Multiline => "Parsing multiline".to_string(),
            ParseError::OddNumberOfHexDigits => "Parsing hex bytearray".to_string(),
            ParseError::Predicate => "Parsing predicate".to_string(),
            ParseError::PredicateValue => "Parsing predicate value".to_string(),
            ParseError::RegexExpr { .. } => "Parsing regex".to_string(),
            ParseError::RequestSection => "Parsing section".to_string(),
            ParseError::RequestSectionName { .. } => "Parsing request section name".to_string(),
            ParseError::ResponseSection => "Parsing section".to_string(),
            ParseError::ResponseSectionName { .. } => "Parsing response section name".to_string(),
            ParseError::Space => "Parsing space".to_string(),
            ParseError::Status => "Parsing status code".to_string(),
            ParseError::TemplateVariable => "Parsing template variable".to_string(),
            ParseError::UrlIllegalCharacter(_) => "Parsing URL".to_string(),
            ParseError::UrlInvalidStart => "Parsing URL".to_string(),
            ParseError::Version => "Parsing version".to_string(),
            ParseError::XPathExpr => "Parsing XPath expression".to_string(),
            ParseError::Xml => "Parsing XML".to_string(),
            // TODO: implement all variants
            // _ => ,
            ParseError::Unicode => format!("{self:?}"),
            ParseError::Url => format!("{self:?}"),
        }
    }

    fn fixme(&self) -> String {
        match &self.inner {
            ParseError::DuplicateSection => "the section is already defined".to_string(),
            ParseError::EscapeChar => "the escaping sequence is not valid".to_string(),
            ParseError::Expecting { value } => format!("expecting '{value}'"),
            ParseError::FileContentType => "expecting a content type".to_string(),
            ParseError::Filename => "expecting a filename".to_string(),
            ParseError::GraphQlVariables => {
                "GraphQL variables is not a valid JSON object".to_string()
            }
            ParseError::HexDigit => "expecting a valid hexadecimal number".to_string(),
            ParseError::InvalidCookieAttribute => "the cookie attribute is not valid".to_string(),
            ParseError::InvalidOption(name) => {
                let valid_values = [
                    "aws-sigv4",
                    "cacert",
                    "cert",
                    "compressed",
                    "connect-to",
                    "delay",
                    "insecure",
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
                    "resolve",
                    "retry",
                    "retry-interval",
                    "skip",
                    "variable",
                    "verbose",
                    "very-verbose",
                ];
                let default = format!("Valid values are {}", valid_values.join(", "));
                let did_you_mean = did_you_mean(&valid_values, name.as_str(), &default);
                format!("the option name is not valid. {did_you_mean}")
            }
            ParseError::Json(variant) => match variant {
                JsonErrorVariant::TrailingComma => "trailing comma is not allowed".to_string(),
                JsonErrorVariant::EmptyElement => {
                    "expecting an element; found empty element instead".to_string()
                }
                JsonErrorVariant::ExpectingElement => {
                    "expecting a boolean, number, string, array, object or null".to_string()
                }
            },
            ParseError::JsonPathExpr => "expecting a JSONPath expression".to_string(),
            ParseError::Method { name } => {
                let valid_values = [
                    "GET", "HEAD", "POST", "PUT", "DELETE", "CONNECT", "OPTIONS", "TRACE", "PATCH",
                ];
                let default = format!("Valid values are {}", valid_values.join(", "));
                let did_you_mean = did_you_mean(&valid_values, name.as_str(), &default);
                format!("the HTTP method <{name}> is not valid. {did_you_mean}")
            }
            ParseError::Multiline => "the multiline is not valid".to_string(),
            ParseError::OddNumberOfHexDigits => {
                "expecting an even number of hex digits".to_string()
            }
            ParseError::Predicate => "expecting a predicate".to_string(),
            ParseError::PredicateValue => "invalid predicate value".to_string(),
            ParseError::RegexExpr { message } => format!("invalid Regex expression: {message}"),
            ParseError::RequestSection => "this is not a valid section for a request".to_string(),
            ParseError::RequestSectionName { name } => {
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
            ParseError::ResponseSection => "this is not a valid section for a response".to_string(),
            ParseError::ResponseSectionName { name } => {
                let valid_values = ["Captures", "Asserts"];
                let default = "Valid values are Captures or Asserts";
                let did_your_mean = did_you_mean(&valid_values, name.as_str(), default);
                format!("the section is not valid. {did_your_mean}")
            }
            ParseError::Space => "expecting a space".to_string(),
            ParseError::Status => "HTTP status code is not valid".to_string(),
            ParseError::TemplateVariable => "expecting a variable".to_string(),
            ParseError::UrlIllegalCharacter(c) => format!("illegal character <{c}>"),
            ParseError::UrlInvalidStart => "expecting http://, https:// or {{".to_string(),
            ParseError::Version => {
                "HTTP version must be HTTP, HTTP/1.0, HTTP/1.1 or HTTP/2".to_string()
            }
            ParseError::XPathExpr => "expecting a XPath expression".to_string(),
            ParseError::Xml => "invalid XML".to_string(),
            // TODO: implement all variants
            // _ => format!("{self:?}"),
            ParseError::Unicode => format!("{self:?}"),
            ParseError::Url => format!("{self:?}"),
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
}
