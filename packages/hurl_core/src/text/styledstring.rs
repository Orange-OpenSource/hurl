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
use colored::Colorize;

use crate::text::style::{Color, Style};

/// A String with style.
///
/// A styled string can be composed of styled parts (tokens). A token has a style (an optional
/// foreground color with a bold attribute) and a string content.
///
/// A styled string can be rendered as plain text, or as a string with
/// [ANSI escape codes](https://en.wikipedia.org/wiki/ANSI_escape_code). It's useful to make text
/// that can be colored or plain at runtime.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct StyledString {
    /// A list of tokens (styled parts).
    tokens: Vec<Token>,
}

/// Represents part of a [`StyledString`].
#[derive(Clone, Debug, PartialEq, Eq)]
struct Token {
    /// The string content of the token.
    content: String,
    /// The style of the token, a foreground color with a bold attribute.
    style: Style,
}

/// The format in which a [`StyledString`] can be rendered.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Format {
    Plain,
    Ansi,
}

impl StyledString {
    /// Creates an empty instance of a styled string.
    pub fn new() -> StyledString {
        StyledString { tokens: vec![] }
    }

    /// Appends a given `content` without any style onto this end of this `StyledString`
    pub fn push(&mut self, content: &str) {
        self.push_with(content, Style::new());
    }

    /// Appends a given `content` with a specific `style`.
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

    /// Renders a styled string given a `format`.
    pub fn to_string(&self, format: Format) -> String {
        self.tokens
            .iter()
            .map(|token| token.to_string(format))
            .collect::<Vec<String>>()
            .join("")
    }

    /// Appends a styled string.
    pub fn append(&mut self, other: StyledString) {
        for token in other.tokens {
            self.push_token(token);
        }
    }

    /// Splits a styled string to a given list of [`StyledString`], given a `delimiter`.
    pub fn split(&self, delimiter: char) -> Vec<StyledString> {
        let mut items = vec![];
        let mut item = StyledString::new();
        for token in &self.tokens {
            let mut substrings = token.content.split(delimiter).collect::<Vec<&str>>();
            let first = substrings.remove(0);
            if !first.is_empty() {
                item.push_with(first, token.style);
            }
            for substring in substrings {
                items.push(item);
                item = StyledString::new();
                if !substring.is_empty() {
                    item.push_with(substring, token.style);
                }
            }
        }
        items.push(item);
        items
    }

    /// Tests if this styled string ends with a given `value`, no matter what's the style of the string.
    pub fn ends_with(&self, value: &str) -> bool {
        self.to_string(Format::Plain).ends_with(value)
    }

    /// Returns the length of visible chars.
    pub fn len(&self) -> usize {
        self.tokens.iter().fold(0, |acc, t| acc + t.content.len())
    }

    /// Checks if this string is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Add newlines so each lines of this string has a maximum of `max_width` chars.
    pub fn wrap(&self, max_width: usize) -> StyledString {
        let mut string = StyledString::new();
        let mut width = 0;

        for token in &self.tokens {
            let mut chunk = String::new();
            let mut it = token.content.chars().peekable();

            // Iterate over each chars of the current token, splitting the current
            // token if necessary
            while let Some(c) = it.next() {
                chunk.push(c);
                width += 1;

                if width >= max_width {
                    let token = Token::new(&chunk, token.style);
                    string.push_token(token);
                    if it.peek().is_some() {
                        // New lines are always plain
                        let nl = Token::new("\n", Style::new());
                        string.push_token(nl);
                    }
                    chunk = String::new();
                    width = 0;
                }
            }

            // Append the last chunk
            if !chunk.is_empty() {
                let token = Token::new(&chunk, token.style);
                string.push_token(token);
            }
        }
        string
    }
}

/// Represents part of a styled string.
impl Token {
    fn new(content: &str, style: Style) -> Token {
        let content = content.to_string();
        Token { content, style }
    }

    fn to_string(&self, format: Format) -> String {
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
                Color::BrightBlack => {
                    if self.style.bold {
                        s.bright_black().bold().to_string()
                    } else {
                        s.bright_black().to_string()
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
                Color::Magenta => {
                    if self.style.bold {
                        s.magenta().bold().to_string()
                    } else {
                        s.magenta().to_string()
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
        crate::text::init_crate_colored();

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

    #[test]
    fn wrap_single_plain_token() {
        let mut line = StyledString::new();
        line.push("aaaabbbbcccc");

        let mut wrapped = StyledString::new();
        wrapped.push("aaaa\nbbbb\ncccc");

        assert_eq!(line.wrap(4), wrapped);
        assert_eq!(line.len(), 12);
        assert_eq!(line.wrap(4).len(), 14);
    }

    #[test]
    fn wrap_single_styled_token() {
        let mut line = StyledString::new();
        line.push_with("aaaabbbbcccc", Style::new().blue());

        let mut wrapped = StyledString::new();
        wrapped.push_with("aaaa", Style::new().blue());
        wrapped.push("\n");
        wrapped.push_with("bbbb", Style::new().blue());
        wrapped.push("\n");
        wrapped.push_with("cccc", Style::new().blue());

        assert_eq!(line.wrap(4), wrapped);
        assert_eq!(line.len(), 12);
        assert_eq!(line.wrap(4).len(), 14);
    }

    #[test]
    fn wrap_multi_styled_token() {
        let mut line = StyledString::new();
        line.push_with("aaa", Style::new().blue());
        line.push_with("ab", Style::new().green());
        line.push_with("bbbccc", Style::new().yellow());
        line.push_with("cee", Style::new().purple());

        let mut wrapped = StyledString::new();
        wrapped.push_with("aaa", Style::new().blue());
        wrapped.push_with("a", Style::new().green());
        wrapped.push("\n");
        wrapped.push_with("b", Style::new().green());
        wrapped.push_with("bbb", Style::new().yellow());
        wrapped.push("\n");
        wrapped.push_with("ccc", Style::new().yellow());
        wrapped.push_with("c", Style::new().purple());
        wrapped.push("\n");
        wrapped.push_with("ee", Style::new().purple());

        assert_eq!(line.wrap(4), wrapped);
    }
}
