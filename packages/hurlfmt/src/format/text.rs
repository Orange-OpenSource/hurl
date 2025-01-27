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
use hurl_core::ast::HurlFile;
use hurl_core::text::{Format, Style, StyledString};

use crate::format::{Token, Tokenizable};

pub fn format(hurl_file: &HurlFile, color: bool) -> String {
    let mut buffer = String::new();
    for token in &hurl_file.tokenize() {
        buffer.push_str(format_token(token, color).as_str());
    }
    buffer
}

pub fn format_token(token: &Token, color: bool) -> String {
    let format = if color { Format::Ansi } else { Format::Plain };

    match token {
        Token::Whitespace(value) => value.clone(),
        Token::Method(value) => {
            let mut s = StyledString::new();
            s.push_with(value, Style::new().yellow());
            s.to_string(format)
        }
        Token::Version(value) => value.clone(),
        Token::Status(value) => value.clone(),
        Token::SectionHeader(value) => {
            let mut s = StyledString::new();
            s.push_with(value, Style::new().magenta());
            s.to_string(format)
        }
        Token::Comment(value) => {
            let mut s = StyledString::new();
            s.push_with(value, Style::new().bright_black());
            s.to_string(format)
        }
        Token::Value(value) => value.clone(),
        Token::Colon(value) => value.clone(),
        Token::QueryType(value) => {
            let mut s = StyledString::new();
            s.push_with(value, Style::new().cyan());
            s.to_string(format)
        }
        Token::PredicateType(value) => {
            let mut s = StyledString::new();
            s.push_with(value, Style::new().yellow());
            s.to_string(format)
        }
        Token::Not(value) => {
            let mut s = StyledString::new();
            s.push_with(value, Style::new().yellow());
            s.to_string(format)
        }
        Token::Boolean(value) | Token::Number(value) => {
            let mut s = StyledString::new();
            s.push_with(value, Style::new().cyan());
            s.to_string(format)
        }
        Token::String(value) => {
            let mut s = StyledString::new();
            s.push_with(value, Style::new().green());
            s.to_string(format)
        }
        Token::StringDelimiter(value) => {
            let mut s = StyledString::new();
            s.push_with(value, Style::new().green());
            s.to_string(format)
        }
        Token::CodeDelimiter(value) => {
            let mut s = StyledString::new();
            s.push_with(value, Style::new().green());
            s.to_string(format)
        }
        Token::CodeVariable(value) => {
            let mut s = StyledString::new();
            s.push_with(value, Style::new().green());
            s.to_string(format)
        }
        Token::Keyword(value) => value.clone(),
        Token::FilterType(value) => {
            let mut s = StyledString::new();
            s.push_with(value, Style::new().cyan());
            s.to_string(format)
        }
        Token::Lang(value) => value.clone(),
        Token::Unit(value) => {
            let mut s = StyledString::new();
            s.push_with(value, Style::new().cyan());
            s.to_string(format)
        }
    }
}
