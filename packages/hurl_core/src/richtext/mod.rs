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

use colored::Colorize;
use std::fmt;
use std::fmt::Formatter;

/// An String with color and style
#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(unused)]
struct RichText<'a> {
    format: Format,
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
    fg: Option<Color>,
    bold: bool,
}

#[allow(unused)]
#[derive(Clone, Debug, PartialEq, Eq)]
enum Color {
    Red,
    Green,
    Blue,
}

#[allow(unused)]
impl<'a> RichText<'a> {
    pub fn new(format: Format) -> RichText<'a> {
        RichText {
            format,
            tokens: vec![],
        }
    }

    pub fn add_token(&mut self, content: &'a str, fg: Option<Color>, bold: bool) {
        let token = Token::new(content, fg, bold);
        self.tokens.push(token);
    }

    pub fn text(mut self, content: &'a str) -> RichText {
        self.add_token(content, None, false);
        self
    }

    pub fn red(mut self, content: &'a str) -> RichText {
        self.add_token(content, Some(Color::Red), false);
        self
    }
}

impl fmt::Display for RichText<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let string = self
            .tokens
            .iter()
            .map(|token| token.to_string(self.format))
            .collect::<Vec<String>>()
            .join("");
        write!(f, "{string}")
    }
}

impl<'a> Token<'a> {
    pub fn new(content: &'a str, fg: Option<Color>, bold: bool) -> Token {
        Token { content, fg, bold }
    }

    // FIXME: same as [`RichText`] we could implement instead `fmt::Display` instead.
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
        if let Some(color) = &self.fg {
            s = match color {
                Color::Red => s.red().to_string(),
                Color::Green => s.green().to_string(),
                Color::Blue => s.blue().to_string(),
            };
        }
        if self.bold {
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
        let message = RichText::new(Format::Plain)
            .text("Hello ")
            .red("Bob")
            .text("!");
        assert_eq!(message.to_string(), "Hello Bob!");

        let message = RichText::new(Format::Ansi)
            .text("Hello ")
            .red("Bob")
            .text("!");
        assert_eq!(message.to_string(), "Hello \u{1b}[31mBob\u{1b}[0m!");
    }
}
