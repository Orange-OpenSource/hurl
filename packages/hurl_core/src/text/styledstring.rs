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
pub struct StyledString {
    tokens: Vec<Token>,
}

#[allow(unused)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Format {
    Plain,
    Ansi,
}

#[allow(unused)]
#[derive(Clone, Debug, PartialEq, Eq)]
struct Token {
    content: String,
    style: Style,
}

#[allow(unused)]
impl StyledString {
    pub fn new() -> StyledString {
        StyledString { tokens: vec![] }
    }

    pub fn push(&mut self, content: &str) {
        self.push_with(content, Style::new());
    }

    pub fn push_with(&mut self, content: &str, style: Style) {
        let token = Token::new(content, style);
        self.push_token(token);
    }

    fn push_token(&mut self, token: Token) {
        // Concatenate content to last token if it has the same style
        if let Some(last) = self.tokens.last_mut() {
            if last.style == token.style {
                last.content.push_str(&token.content);
                return;
            }
        }
        self.tokens.push(token);
    }

    pub fn to_string(&self, format: Format) -> String {
        self.tokens
            .iter()
            .map(|token| token.to_string(format))
            .collect::<Vec<String>>()
            .join("")
    }

    pub fn append(&mut self, other: StyledString) {
        for token in other.tokens {
            self.push_token(token);
        }
    }

    pub fn split(&self, delimiter: char) -> Vec<StyledString> {
        let mut items = vec![];
        let mut item = StyledString::new();
        for token in &self.tokens {
            let mut substrings = token.content.split(delimiter).collect::<Vec<&str>>();
            let first = substrings.remove(0);
            if !first.is_empty() {
                item.push_with(first, token.style.clone());
            }
            for substring in substrings {
                items.push(item);
                item = StyledString::new();
                if !substring.is_empty() {
                    item.push_with(substring, token.style.clone());
                }
            }
        }
        items.push(item);
        items
    }

    pub fn ends_with(&self, value: &str) -> bool {
        self.to_string(Format::Plain).ends_with(value)
    }
}

impl Token {
    pub fn new(content: &str, style: Style) -> Token {
        let content = content.to_string();
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
                Color::Cyan => {
                    if self.style.bold {
                        s.cyan().bold().to_string()
                    } else {
                        s.cyan().to_string()
                    }
                }
                Color::Green => {
                    if self.style.bold {
                        s.green().bold().to_string()
                    } else {
                        s.green().to_string()
                    }
                }
                Color::Purple => {
                    if self.style.bold {
                        s.purple().bold().to_string()
                    } else {
                        s.purple().to_string()
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
    fn test_push() {
        let mut message = StyledString::new();
        message.push("Hello");
        message.push(" ");
        message.push_with("Bob", Style::new().red());
        message.push("!");

        assert_eq!(
            message,
            StyledString {
                tokens: vec![
                    Token {
                        content: "Hello ".to_string(),
                        style: Style::new()
                    },
                    Token {
                        content: "Bob".to_string(),
                        style: Style::new().red()
                    },
                    Token {
                        content: "!".to_string(),
                        style: Style::new()
                    },
                ],
            }
        );
    }

    #[test]
    fn test_append() {
        let mut message1 = StyledString::new();
        message1.push("Hello ");
        message1.push_with("Bob", Style::new().red());
        message1.push("!");
        let mut message2 = StyledString::new();
        message2.push("Hi ");
        message2.push_with("Bill", Style::new().red());
        message2.push("!");

        let mut messages = StyledString::new();
        messages.push("Hello ");
        messages.push_with("Bob", Style::new().red());
        messages.push("!");
        messages.push("Hi ");
        messages.push_with("Bill", Style::new().red());
        messages.push("!");

        message1.append(message2);
        assert_eq!(message1, messages);
    }

    #[test]
    fn test_split() {
        let mut line = StyledString::new();
        line.push("Hello,Hi,");
        line.push_with("Hola", Style::new().red());
        line.push(",Bye,");
        line.push_with("Adios", Style::new().red());

        let mut item0 = StyledString::new();
        item0.push("Hello");
        let mut item1 = StyledString::new();
        item1.push("Hi");
        let mut item2 = StyledString::new();
        item2.push_with("Hola", Style::new().red());
        let mut item3 = StyledString::new();
        item3.push("Bye");
        let mut item4 = StyledString::new();
        item4.push_with("Adios", Style::new().red());
        assert_eq!(line.split(','), vec![item0, item1, item2, item3, item4]);

        // Test empty items
        let mut line = StyledString::new();
        line.push("0,,2,");

        let mut item0 = StyledString::new();
        item0.push("0");
        let item1 = StyledString::new();
        let mut item2 = StyledString::new();
        item2.push("2");
        let item3 = StyledString::new();
        assert_eq!(line.split(','), vec![item0, item1, item2, item3]);
    }

    #[test]
    fn test_ends_with() {
        let mut line = StyledString::new();
        line.push("Hello,Hi,");
        assert!(line.ends_with(","));
        assert!(!line.ends_with("\n"));
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
