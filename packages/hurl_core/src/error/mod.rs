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
use super::ast::SourceInfo;
use super::parser;
use super::parser::ParseError;

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
            ParseError::Method { .. } => "Parsing Method".to_string(),
            ParseError::Version { .. } => "Parsing Version".to_string(),
            ParseError::Status { .. } => "Parsing Status".to_string(),
            ParseError::Filename { .. } => "Parsing Filename".to_string(),
            ParseError::Expecting { .. } => "Parsing literal".to_string(),
            ParseError::Space { .. } => "Parsing space".to_string(),
            ParseError::SectionName { .. } => "Parsing section name".to_string(),
            ParseError::JsonpathExpr { .. } => "Parsing jsonpath expression".to_string(),
            ParseError::XPathExpr { .. } => "Parsing xpath expression".to_string(),
            ParseError::TemplateVariable { .. } => "Parsing template variable".to_string(),
            ParseError::Json { .. } => "Parsing json".to_string(),
            ParseError::Predicate { .. } => "Parsing predicate".to_string(),
            ParseError::PredicateValue { .. } => "Parsing predicate value".to_string(),
            ParseError::RegexExpr { .. } => "Parsing regex".to_string(),
            ParseError::DuplicateSection { .. } => "Parsing section".to_string(),
            ParseError::RequestSection { .. } => "Parsing section".to_string(),
            ParseError::ResponseSection { .. } => "Parsing section".to_string(),
            ParseError::EscapeChar { .. } => "Parsing escape character".to_string(),
            ParseError::InvalidCookieAttribute { .. } => "Parsing cookie attribute".to_string(),
            ParseError::OddNumberOfHexDigits { .. } => "Parsing hex bytearray".to_string(),
            _ => format!("{:?}", self),
        }
    }

    fn fixme(&self) -> String {
        match self.inner.clone() {
            ParseError::Method { .. } => "Available HTTP Method GET, POST, ...".to_string(),
            ParseError::Version { .. } => "The http version must be 1.0, 1.1, 2 or *".to_string(),
            ParseError::Status { .. } => "The http status is not valid".to_string(),
            ParseError::Filename { .. } => "expecting a filename".to_string(),
            ParseError::Expecting { value } => format!("expecting '{}'", value),
            ParseError::Space { .. } => "expecting a space".to_string(),
            ParseError::SectionName { name } => format!("the section {} is not valid", name),
            ParseError::JsonpathExpr { .. } => "expecting a jsonpath expression".to_string(),
            ParseError::XPathExpr { .. } => "expecting a xpath expression".to_string(),
            ParseError::TemplateVariable { .. } => "expecting a variable".to_string(),
            ParseError::Json { .. } => "json error".to_string(),
            ParseError::Predicate { .. } => "expecting a predicate".to_string(),
            ParseError::PredicateValue { .. } => "invalid predicate value".to_string(),
            ParseError::RegexExpr { .. } => "Invalid Regex expression".to_string(),
            ParseError::DuplicateSection { .. } => "The section is already defined".to_string(),
            ParseError::RequestSection { .. } => {
                "This is not a valid section for a request".to_string()
            }
            ParseError::ResponseSection { .. } => {
                "This is not a valid section for a response".to_string()
            }
            ParseError::EscapeChar { .. } => "The escaping sequence is not valid".to_string(),
            ParseError::InvalidCookieAttribute { .. } => {
                "The cookie attribute is not valid".to_string()
            }
            ParseError::OddNumberOfHexDigits { .. } => {
                "Expecting an even number of hex digits".to_string()
            }
            _ => format!("{:?}", self),
        }
    }
}
