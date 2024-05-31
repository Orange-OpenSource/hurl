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

use crate::text::style::{Color, Style};
use colored::Colorize;

/// A String with style
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[allow(unused)]
pub struct StyledString<'a> {
    tokens: Vec<Token<'a>>,
}

#[allow(unused)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Format {
    Plain,
    Ansi,
}

#[allow(unused)]
#[derive(Clone, Debug, PartialEq, Eq)]
struct Token<'a> {
    content: &'a str,
    style: Style,
}

#[allow(unused)]
impl<'a> StyledString<'a> {
    pub fn new() -> StyledString<'a> {
        StyledString { tokens: vec![] }
    }

    pub fn push(&mut self, content: &'a str) {
        let style = Style::new();
        let token = Token::new(content, style);
        self.tokens.push(token);
    }

    pub fn push_with(&mut self, content: &'a str, style: Style) {
        let token = Token::new(content, style);
        self.tokens.push(token);
    }

    pub fn to_string(&self, format: Format) -> String {
        self.tokens
            .iter()
            .map(|token| token.to_string(format))
            .collect::<Vec<String>>()
            .join("")
    }
}

impl<'a> Token<'a> {
    pub fn new(content: &'a str, style: Style) -> Token {
        Token { content, style }
    }

    pub fn to_string(&self, format: Format) -> String {
        match format {
            Format::Plain => self.plain(),
            Format::Ansi => self.ansi(),
        }
    }

    fn plain(&self) -> String {
        self.content.to_string()
    }

    fn ansi(&self) -> String {
        let mut s = self.content.to_string();
        if let Some(color) = &self.style.fg {
            s = match color {
                Color::Blue => {
                    if self.style.bold {
                        s.blue().bold().to_string()
                    } else {
                        s.blue().to_string()
                    }
                }
                Color::Green => {
                    if self.style.bold {
                        s.green().bold().to_string()
                    } else {
                        s.green().to_string()
                    }
                }
                Color::Red => {
                    if self.style.bold {
                        s.red().bold().to_string()
                    } else {
                        s.red().to_string()
                    }
                }
                Color::Yellow => {
                    if self.style.bold {
                        s.yellow().bold().to_string()
                    } else {
                        s.yellow().to_string()
                    }
                }
            };
        } else if self.style.bold {
            s = s.bold().to_string();
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        colored::control::set_override(true);
        let mut message = StyledString::new();
        message.push("Hello ");
        message.push_with("Bob", Style::new().red());
        message.push("!");
        assert_eq!(message.to_string(Format::Plain), "Hello Bob!");
        assert_eq!(
            message.to_string(Format::Ansi),
            "Hello \u{1b}[31mBob\u{1b}[0m!"
        );
    }

    #[test]
    fn compare_with_crate_colored() {
        // These tests are used to check regression against the [colored crate](https://crates.io/crates/colored).
        // A short-term objective is to remove the colored crates to manage ansi colors.
        let mut message = StyledString::new();
        message.push_with("foo", Style::new().red().bold());
        assert_eq!(
            "foo".red().bold().to_string(),
            message.to_string(Format::Ansi),
        );

        let mut message = StyledString::new();
        message.push_with("bar", Style::new().bold());
        assert_eq!("bar".bold().to_string(), message.to_string(Format::Ansi),);
    }
}
