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
use std::cmp::min;

/// Represents a text reader.
///
/// The `Reader` implements methods to read a stream of text. A reader manages
/// an internal `state` which is the position of the current cursor within the reader's buffer.
/// Methods like [`Reader::read`], [`Reader::read_while`]
/// do advance the internal reader's`state`. Other methods, like [`Reader::peek`], [`Reader::peek_n`]
/// allows to get the next chars in the buffer without modifying the current reader state.
///
/// # Example
/// ```
///  use hurl_core::reader::Reader;
///
///  let mut reader = Reader::new("hi");
///  let state = reader.state.cursor; // cursor is 0
///  let eof = reader.is_eof();
///  let val = reader.peek_n(2); // val = "hi"
///  let val = reader.read().unwrap(); // val = 'h'
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Reader {
    pub buffer: Vec<char>,
    pub state: ReaderState,
}

/// Represents a line and column position in a reader.
///
/// Index are 1-based.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Pos {
    pub line: usize,
    pub column: usize,
}

impl Pos {
    /// Creates a new position.
    pub fn new(line: usize, column: usize) -> Pos {
        Pos { line, column }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ReaderState {
    pub cursor: usize,
    pub pos: Pos,
}

impl Reader {
    /// Creates a new reader.
    pub fn new(s: &str) -> Reader {
        Reader {
            buffer: s.chars().collect(),
            state: ReaderState {
                cursor: 0,
                pos: Pos { line: 1, column: 1 },
            },
        }
    }

    /// Returns true if the reader has read all the buffer, false otherwise.
    pub fn is_eof(&self) -> bool {
        self.state.cursor == self.buffer.len()
    }

    /// Returns the next char from the buffer advancing the internal state.
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

    /// Returns `count` chars from the buffer advancing the internal state.
    /// This methods can returns less than `count` chars if there is not enough chars in the buffer.
    pub fn read_n(&mut self, count: usize) -> String {
        let mut s = String::new();
        for _ in 0..count {
            match self.read() {
                None => {}
                Some(c) => s.push(c),
            }
        }
        s
    }

    /// Returns chars from the buffer while `predicate` is true, advancing the internal state.
    pub fn read_while(&mut self, predicate: fn(&char) -> bool) -> String {
        let mut s = String::new();
        loop {
            match self.peek() {
                None => return s,
                Some(c) => {
                    if predicate(&c) {
                        s.push(self.read().unwrap());
                    } else {
                        return s;
                    }
                }
            }
        }
    }

    /// Returns the next char from the buffer without advancing the internal state.
    pub fn peek(&self) -> Option<char> {
        self.buffer.get(self.state.cursor).copied()
    }

    /// Returns the next char ignoring whitespace without advancing the internal state.
    pub fn peek_ignoring_whitespace(&self) -> Option<char> {
        let mut i = self.state.cursor;
        loop {
            if let Some(c) = self.buffer.get(i).copied() {
                if c != ' ' && c != '\t' && c != '\n' && c != '\r' {
                    return Some(c);
                }
            } else {
                return None;
            }
            i += 1;
        }
    }

    /// Returns the `count` char from the buffer without advancing the internal state.
    /// This methods can returns less than `count` chars if there is not enough chars in the buffer.
    pub fn peek_n(&self, count: usize) -> String {
        let start = self.state.cursor;
        let end = min(start + count, self.buffer.len());
        self.buffer[start..end].iter().collect()
    }

    pub fn peek_back(&self, start: usize) -> String {
        let end = self.state.cursor;
        self.buffer[start..end].iter().collect()
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
        let mut reader = Reader::new("hi");
        assert_eq!(reader.state.cursor, 0);
        assert!(!reader.is_eof());
        assert_eq!(reader.peek_n(2), "hi".to_string());

        assert_eq!(reader.read().unwrap(), 'h');
        assert_eq!(reader.state.cursor, 1);
        assert_eq!(reader.peek().unwrap(), 'i');
        assert_eq!(reader.state.cursor, 1);
        assert_eq!(reader.read().unwrap(), 'i');
        assert!(reader.is_eof());
        assert_eq!(reader.read(), None);
    }
}
