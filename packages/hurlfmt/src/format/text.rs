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
use colored::*;

use hurl_core::ast::*;

use super::token::*;

pub fn format(hurl_file: HurlFile, color: bool) -> String {
    let mut buffer = String::from("");
    for token in hurl_file.tokenize() {
        buffer.push_str(format_token(token, color).as_str());
    }
    buffer
}

pub fn format_token(token: Token, color: bool) -> String {
    match token {
        Token::Whitespace(value) => value,
        Token::Method(value) => {
            if color {
                value.yellow().to_string()
            } else {
                value
            }
        }
        Token::Version(value) => value,
        Token::Status(value) => value,
        Token::SectionHeader(value) => {
            if color {
                value.magenta().to_string()
            } else {
                value
            }
        }
        Token::Comment(value) => {
            if color {
                value.bright_black().to_string()
            } else {
                value
            }
        }
        Token::Value(value) => value,
        Token::Colon(value) => value,
        Token::QueryType(value) => {
            if color {
                value.cyan().to_string()
            } else {
                value
            }
        }
        Token::PredicateType(value) => {
            if color {
                value.yellow().to_string()
            } else {
                value
            }
        }
        Token::Not(value) => {
            if color {
                value.yellow().to_string()
            } else {
                value
            }
        }
        Token::Boolean(value) | Token::Number(value) => {
            if color {
                value.cyan().to_string()
            } else {
                value
            }
        }
        Token::String(value) => {
            if color {
                value.green().to_string()
            } else {
                value
            }
        }
        Token::Quote(value) => {
            if color {
                value.green().to_string()
            } else {
                value
            }
        }
        Token::CodeDelimiter(value) => {
            if color {
                value.green().to_string()
            } else {
                value
            }
        }
        Token::CodeVariable(value) => {
            if color {
                value.green().to_string()
            } else {
                value
            }
        }
        Token::Keyword(value) => value,
    }
}
