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
use super::ast::SourceInfo;
use super::parser;
use super::parser::ParseError;
use core::cmp;

pub trait Error {
    fn source_info(&self) -> SourceInfo;
    fn description(&self) -> String;
    fn fixme(&self) -> String;
}

impl Error for parser::Error {
    fn source_info(&self) -> SourceInfo {
        SourceInfo {
            start: self.pos.clone(),
            end: self.pos.clone(),
        }
    }

    fn description(&self) -> String {
        match self.clone().inner {
            ParseError::Method { .. } => "Parsing method".to_string(),
            ParseError::Version { .. } => "Parsing version".to_string(),
            ParseError::Status { .. } => "Parsing status code".to_string(),
            ParseError::Filename { .. } => "Parsing filename".to_string(),
            ParseError::Expecting { .. } => "Parsing literal".to_string(),
            ParseError::Space { .. } => "Parsing space".to_string(),
            ParseError::RequestSectionName { .. } => "Parsing request section name".to_string(),
            ParseError::ResponseSectionName { .. } => "Parsing response section name".to_string(),
            ParseError::JsonpathExpr { .. } => "Parsing JSONPath expression".to_string(),
            ParseError::XPathExpr { .. } => "Parsing XPath expression".to_string(),
            ParseError::TemplateVariable { .. } => "Parsing template variable".to_string(),
            ParseError::Json { .. } => "Parsing JSON".to_string(),
            ParseError::Predicate { .. } => "Parsing predicate".to_string(),
            ParseError::PredicateValue { .. } => "Parsing predicate value".to_string(),
            ParseError::RegexExpr { .. } => "Parsing regex".to_string(),
            ParseError::DuplicateSection { .. } => "Parsing section".to_string(),
            ParseError::RequestSection { .. } => "Parsing section".to_string(),
            ParseError::ResponseSection { .. } => "Parsing section".to_string(),
            ParseError::EscapeChar { .. } => "Parsing escape character".to_string(),
            ParseError::InvalidCookieAttribute { .. } => "Parsing cookie attribute".to_string(),
            ParseError::OddNumberOfHexDigits { .. } => "Parsing hex bytearray".to_string(),
            ParseError::UrlIllegalCharacter(_) => "Parsing URL".to_string(),
            _ => format!("{:?}", self),
        }
    }

    fn fixme(&self) -> String {
        match self.inner.clone() {
            ParseError::Method { name }
            => format!("the HTTP method is not valid. {}", did_you_mean(
                &["GET", "HEAD", "POST", "PUT", "DELETE", "CONNECT", "OPTIONS", "TRACE", "PATCH"],
                name.as_str(),
                "Available HTTP Methods are GET, HEAD, POST, PUT, DELETE, CONNECT, OPTIONS, TRACE or PATCH",
            )),
            ParseError::Version { .. } => "HTTP version must be 1.0, 1.1, 2 or *".to_string(),
            ParseError::Status { .. } => "HTTP status code is not valid".to_string(),
            ParseError::Filename { .. } => "expecting a filename".to_string(),
            ParseError::Expecting { value } => format!("expecting '{}'", value),
            ParseError::Space { .. } => "expecting a space".to_string(),
            ParseError::RequestSectionName { name }
            => format!("the section is not valid. {}", did_you_mean(
                &["QueryStringParams", "FormParams", "MultipartFormData", "Cookies"],
                name.as_str(),
                "Valid values are QueryStringParams, FormParams, MultipartFormData or Cookies",
            )),
            ParseError::ResponseSectionName { name }
            => format!("the section is not valid. {}", did_you_mean(
                &["Captures", "Asserts"],
                name.as_str(),
                "Valid values are Captures or Asserts",
            )),
            ParseError::JsonpathExpr { .. } => "expecting a JSONPath expression".to_string(),
            ParseError::XPathExpr { .. } => "expecting a XPath expression".to_string(),
            ParseError::TemplateVariable { .. } => "expecting a variable".to_string(),
            ParseError::Json { .. } => "JSON error".to_string(),
            ParseError::Predicate { .. } => "expecting a predicate".to_string(),
            ParseError::PredicateValue { .. } => "invalid predicate value".to_string(),
            ParseError::RegexExpr { message } => format!("invalid Regex expression: {}", message),
            ParseError::DuplicateSection { .. } => "the section is already defined".to_string(),
            ParseError::RequestSection { .. } => {
                "this is not a valid section for a request".to_string()
            }
            ParseError::ResponseSection { .. } => {
                "this is not a valid section for a response".to_string()
            }
            ParseError::EscapeChar { .. } => "the escaping sequence is not valid".to_string(),
            ParseError::InvalidCookieAttribute { .. } => {
                "the cookie attribute is not valid".to_string()
            }
            ParseError::OddNumberOfHexDigits { .. } => {
                "expecting an even number of hex digits".to_string()
            }
            ParseError::UrlIllegalCharacter(c) => format!("illegal character <{}>", c),
            _ => format!("{:?}", self),
        }
    }
}

fn did_you_mean(valid_values: &[&str], actual: &str, default: &str) -> String {
    if let Some(suggest) = suggestion(valid_values, actual) {
        format!("Did you mean {}?", suggest)
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

// from https://en.wikibooks.org/wiki/Algorithm_Implementation/Strings/Levenshtein_distance#Rust
fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let v1: Vec<char> = s1.chars().collect();
    let v2: Vec<char> = s2.chars().collect();

    fn min3<T: Ord>(v1: T, v2: T, v3: T) -> T {
        cmp::min(v1, cmp::min(v2, v3))
    }
    fn delta(x: char, y: char) -> usize {
        if x == y {
            0
        } else {
            1
        }
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
