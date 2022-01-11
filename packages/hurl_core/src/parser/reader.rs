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
use crate::ast::Pos;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Reader {
    pub buffer: Vec<char>,
    pub state: ReaderState,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReaderState {
    pub cursor: usize,
    pub pos: Pos,
}

impl Reader {
    pub fn init(s: &str) -> Reader {
        Reader {
            buffer: s.chars().collect(),
            state: ReaderState {
                cursor: 0,
                pos: Pos { line: 1, column: 1 },
            },
        }
    }

    pub fn is_eof(&self) -> bool {
        self.state.cursor == self.buffer.len()
    }

    pub fn read(&mut self) -> Option<char> {
        match self.buffer.get(self.state.cursor) {
            None => None,
            Some(c) => {
                self.state.cursor += 1;
                if !is_combining_character(*c) {
                    self.state.pos.column += 1;
                }
                if *c == '\n' {
                    self.state.pos.column = 1;
                    self.state.pos.line += 1;
                }
                Some(*c)
            }
        }
    }

    pub fn peek(&mut self) -> Option<char> {
        self.buffer.get(self.state.cursor).copied()
    }

    pub fn read_while(&mut self, predicate: fn(&char) -> bool) -> String {
        let mut s = String::from("");
        loop {
            match self.peek() {
                None => return s,
                Some(c) => {
                    if predicate(&c) {
                        s.push(self.read().unwrap())
                    } else {
                        return s;
                    }
                }
            }
        }
    }

    // only support escaped spaces for now
    pub fn read_while_escaping(&mut self, predicate: fn(&char) -> bool) -> String {
        let mut s = String::from("");
        let mut escaped = false;
        loop {
            match self.peek() {
                None => return s,
                Some(c) => {
                    if escaped && c == ' ' {
                        escaped = false;
                        s.push(self.read().unwrap())
                    } else if c == '\\' {
                        escaped = true;
                        let _backslash = self.read().unwrap();
                    } else if predicate(&c) {
                        s.push(self.read().unwrap())
                    } else {
                        return s;
                    }
                }
            }
        }
    }

    // assume that you still have count characters to read in your buffer
    pub fn read_n(&mut self, count: usize) -> String {
        let mut s = String::from("");
        for _ in 0..count {
            match self.read() {
                None => {}
                Some(c) => s.push(c),
            }
        }
        s
    }

    pub fn try_literal(&mut self, value: &str) -> bool {
        if self.remaining().starts_with(value.to_string().as_str()) {
            self.read_n(value.len());
            true
        } else {
            false
        }
    }

    pub fn remaining(&self) -> String {
        self.buffer.as_slice()[self.state.cursor..self.buffer.len()]
            .iter()
            .collect()
    }

    pub fn slice(&self, start: usize, end: usize) -> String {
        self.buffer.as_slice()[start..end].iter().collect()
    }

    pub fn from(&self, start: usize) -> String {
        let end = self.state.cursor;
        self.buffer.as_slice()[start..end].iter().collect()
    }
}

fn is_combining_character(c: char) -> bool {
    c > '\u{0300}' && c < '\u{036F}' // Combining Diacritical Marks (0300–036F)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reader() {
        let mut reader = Reader::init("hi");
        assert_eq!(reader.state.cursor, 0);
        assert!(!reader.is_eof());
        assert_eq!(reader.remaining(), "hi".to_string());

        assert_eq!(reader.read().unwrap(), 'h');
        assert_eq!(reader.state.cursor, 1);
        assert_eq!(reader.peek().unwrap(), 'i');
        assert_eq!(reader.state.cursor, 1);
        assert_eq!(reader.read().unwrap(), 'i');
        assert!(reader.is_eof());
        assert_eq!(reader.read(), None);
    }

    #[test]
    fn test_try_predicate() {
        let mut reader = Reader::init("hi");
        assert!(reader.try_literal("hi"));
        assert_eq!(reader.state.cursor, 2);

        let mut reader = Reader::init("hello");
        assert!(!reader.try_literal("hi"));
        assert_eq!(reader.state.cursor, 0);
    }
}
