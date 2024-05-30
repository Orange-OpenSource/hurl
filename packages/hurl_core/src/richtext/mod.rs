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

/// A String with color and style
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[allow(unused)]
pub struct RichText<'a> {
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
pub enum Color {
    Red,
    Green,
    Blue,
}

#[allow(unused)]
impl<'a> RichText<'a> {
    pub fn new() -> RichText<'a> {
        RichText { tokens: vec![] }
    }

    pub fn rich(mut self, content: &'a str, fg: Option<Color>, bold: bool) -> RichText {
        let token = Token::new(content, fg, bold);
        self.tokens.push(token);
        self
    }

    pub fn text(mut self, content: &'a str) -> RichText {
        let token = Token::new(content, None, false);
        self.tokens.push(token);
        self
    }

    pub fn red(mut self) -> RichText<'a> {
        if let Some(token) = self.tokens.last_mut() {
            token.fg = Some(Color::Red);
        };
        self
    }

    pub fn bold(mut self) -> RichText<'a> {
        if let Some(token) = self.tokens.last_mut() {
            token.bold = true;
        };
        self
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
    pub fn new(content: &'a str, fg: Option<Color>, bold: bool) -> Token {
        Token { content, fg, bold }
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
        if let Some(color) = &self.fg {
            s = match color {
                Color::Red => {
                    if self.bold {
                        s.red().bold().to_string()
                    } else {
                        s.red().to_string()
                    }
                }
                Color::Green => {
                    if self.bold {
                        s.green().bold().to_string()
                    } else {
                        s.green().to_string()
                    }
                }
                Color::Blue => {
                    if self.bold {
                        s.blue().bold().to_string()
                    } else {
                        s.blue().to_string()
                    }
                }
            };
        } else if self.bold {
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
        let message = RichText::new().text("Hello ").text("Bob").red().text("!");
        assert_eq!(message.to_string(Format::Plain), "Hello Bob!");
        assert_eq!(
            message.to_string(Format::Ansi),
            "Hello \u{1b}[31mBob\u{1b}[0m!"
        );
    }

    #[test]
    fn compare_with_colored() {
        assert_eq!(
            "foo".red().bold().to_string(),
            RichText::new()
                .text("foo")
                .red()
                .bold()
                .to_string(Format::Ansi),
        );

        assert_eq!(
            "foo".red().bold().to_string(),
            RichText::new()
                .rich("foo", Some(Color::Red), true)
                .to_string(Format::Ansi),
        );

        assert_eq!(
            "bar".bold().to_string(),
            RichText::new().text("bar").bold().to_string(Format::Ansi),
        );
    }
}
