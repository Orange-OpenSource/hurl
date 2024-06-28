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
use super::Pos;

/// Represents a JSONPath reader.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Reader {
    pub buffer: Vec<char>,
    pub state: ReaderState,
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
    pub fn peek(&mut self) -> Option<char> {
        self.buffer.get(self.state.cursor).copied()
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

        assert_eq!(reader.read().unwrap(), 'h');
        assert_eq!(reader.state.cursor, 1);
        assert_eq!(reader.peek().unwrap(), 'i');
        assert_eq!(reader.state.cursor, 1);
        assert_eq!(reader.read().unwrap(), 'i');
        assert!(reader.is_eof());
        assert_eq!(reader.read(), None);
    }
}
