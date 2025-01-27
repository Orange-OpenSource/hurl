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
//! Represents a text reader.

/// The `Reader` implements methods to read a stream of text. A reader manages
/// an internal `cursor` : it's the current read index position within the reader's internal buffer.
///
/// Methods like [`Reader::read`], [`Reader::read_while`] do advance the internal reader's `cursor`.
/// Other methods, like [`Reader::peek`], [`Reader::peek_n`] allows to get the next chars in the
/// buffer without modifying the current reader cursor.
///
/// The cursor is composed of an offset, which is always related to the reader internal buffer.
/// Along the buffer offset, a position [`Pos`] is updated each time a char is read. This position
/// corresponds to the column and row index in the buffer document. In most of the case, the
/// position is initialized to the first char, but a reader instance can be created using
/// [`Reader::with_pos`] to set a given started position. This can be useful when a reader
/// is instantiated as a "sub reader" of a given reader, and we want to report position relatively
/// to the main reader (for errors but also for constructed structures).
///
/// # Example
/// ```
///  use hurl_core::reader::Reader;
///
///  let mut reader = Reader::new("hi");
///  assert_eq!(reader.cursor().index, 0);
///  assert!(!reader.is_eof());
///  assert_eq!(reader.peek_n(2), "hi".to_string());
///  assert_eq!(reader.read(), Some('h'));
///  assert_eq!(reader.cursor().index, 1);
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Reader {
    buf: Vec<char>,
    cursor: Cursor,
}

/// Represents a line and column position in a reader.
///
/// Indices are 1-based.
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

/// A position in a text buffer.
///
/// The position has two components: a char `offset` in the internal buffer of the reader, and
/// a column-row oriented position `pos`, used for human display. `pos` is usually initialized to
/// the first char of the buffer but it can also be set with a position inside another reader. This
/// allows the report of error of a sub-reader, relative to a parent reader.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Cursor {
    pub index: usize,
    pub pos: Pos,
}

impl Reader {
    /// Creates a new reader, position of the index is at the first char.
    pub fn new(s: &str) -> Self {
        Reader {
            buf: s.chars().collect(),
            cursor: Cursor {
                index: 0,
                pos: Pos { line: 1, column: 1 },
            },
        }
    }

    /// Creates a new reader, `pos` is position of the index: this allow to report created
    /// structures and error to be referenced from this position.
    ///
    /// Note: the `buffer` offset is still initialized to 0.
    pub fn with_pos(s: &str, pos: Pos) -> Self {
        Reader {
            buf: s.chars().collect(),
            cursor: Cursor { index: 0, pos },
        }
    }

    /// Returns the current position of the read index.
    pub fn cursor(&self) -> Cursor {
        self.cursor
    }

    /// Position the read index to a new position.
    pub fn seek(&mut self, to: Cursor) {
        self.cursor = to;
    }

    /// Returns true if the reader has read all the buffer, false otherwise.
    pub fn is_eof(&self) -> bool {
        self.cursor.index == self.buf.len()
    }

    /// Returns the next char from the buffer advancing the internal state.
    pub fn read(&mut self) -> Option<char> {
        match self.buf.get(self.cursor.index) {
            None => None,
            Some(c) => {
                self.cursor.index += 1;
                if !is_combining_character(*c) {
                    self.cursor.pos.column += 1;
                }
                if *c == '\n' {
                    self.cursor.pos.column = 1;
                    self.cursor.pos.line += 1;
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
    pub fn read_while(&mut self, predicate: fn(char) -> bool) -> String {
        let mut s = String::new();
        loop {
            match self.peek() {
                None => return s,
                Some(c) => {
                    if predicate(c) {
                        _ = self.read();
                        s.push(c);
                    } else {
                        return s;
                    }
                }
            }
        }
    }

    /// Reads a string from a `start` position to the current position (excluded).
    ///
    /// This method doesn't modify the read index since we're reading "backwards" to the current
    /// read index.
    pub fn read_from(&self, start: usize) -> String {
        let end = self.cursor.index;
        self.buf[start..end].iter().collect()
    }

    /// Peeks the next char from the buffer without advancing the internal state.
    pub fn peek(&self) -> Option<char> {
        self.buf.get(self.cursor.index).copied()
    }

    /// Peeks the next char that meet a `predicate`.
    pub fn peek_if(&self, predicate: fn(char) -> bool) -> Option<char> {
        let mut i = self.cursor.index;
        loop {
            let &c = self.buf.get(i)?;
            if predicate(c) {
                return Some(c);
            }
            i += 1;
        }
    }

    /// Peeks a string of `count` char without advancing the internal state.
    /// This methods can return less than `count` chars if there is not enough chars in the buffer.
    pub fn peek_n(&self, count: usize) -> String {
        let start = self.cursor.index;
        let end = (start + count).min(self.buf.len());
        self.buf[start..end].iter().collect()
    }
}

fn is_combining_character(c: char) -> bool {
    c > '\u{0300}' && c < '\u{036F}' // Combining Diacritical Marks (0300–036F)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_reader() {
        let mut reader = Reader::new("hi");
        assert_eq!(reader.cursor().index, 0);
        assert!(!reader.is_eof());
        assert_eq!(reader.peek_n(2), "hi".to_string());
        assert_eq!(reader.cursor().index, 0);

        assert_eq!(reader.read().unwrap(), 'h');
        assert_eq!(reader.cursor().index, 1);
        assert_eq!(reader.peek().unwrap(), 'i');
        assert_eq!(reader.cursor().index, 1);
        assert_eq!(reader.read().unwrap(), 'i');
        assert!(reader.is_eof());
        assert_eq!(reader.read(), None);
    }

    #[test]
    fn peek_back() {
        let mut reader = Reader::new("abcdefgh");
        assert_eq!(reader.read(), Some('a'));
        assert_eq!(reader.read(), Some('b'));
        assert_eq!(reader.read(), Some('c'));
        assert_eq!(reader.read(), Some('d'));
        assert_eq!(reader.read(), Some('e'));
        assert_eq!(reader.peek(), Some('f'));
        assert_eq!(reader.read_from(3), "de");
    }

    #[test]
    fn read_while() {
        let mut reader = Reader::new("123456789");
        assert_eq!(reader.read_while(|c| c.is_numeric()), "123456789");
        assert_eq!(reader.cursor().index, 9);
        assert!(reader.is_eof());

        let mut reader = Reader::new("123456789abcde");
        assert_eq!(reader.read_while(|c| c.is_numeric()), "123456789");
        assert_eq!(reader.cursor().index, 9);
        assert!(!reader.is_eof());

        let mut reader = Reader::new("abcde123456789");
        assert_eq!(reader.read_while(|c| c.is_numeric()), "");
        assert_eq!(reader.cursor().index, 0);
    }

    #[test]
    fn reader_create_with_from_pos() {
        let mut main_reader = Reader::new("aaabb");
        _ = main_reader.read();
        _ = main_reader.read();
        _ = main_reader.read();

        let pos = main_reader.cursor().pos;
        let s = main_reader.read_while(|_| true);
        let mut sub_reader = Reader::with_pos(&s, pos);
        assert_eq!(
            sub_reader.cursor,
            Cursor {
                index: 0,
                pos: Pos::new(1, 4)
            }
        );

        _ = sub_reader.read();
        assert_eq!(
            sub_reader.cursor,
            Cursor {
                index: 1,
                pos: Pos::new(1, 5)
            }
        );
    }

    #[test]
    fn peek_ignoring_whitespace() {
        fn is_whitespace(c: char) -> bool {
            c == ' ' || c == '\t'
        }
        let reader = Reader::new("\t\t\tabc");
        assert_eq!(reader.peek_if(|c| !is_whitespace(c)), Some('a'));

        let reader = Reader::new("foo");
        assert_eq!(reader.peek_if(|c| !is_whitespace(c)), Some('f'));
    }
}
