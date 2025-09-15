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
use std::cmp::PartialEq;
use std::{fmt, io};

/// A fast zero dependency JSON formatter / pretty printer.
/// This is a fast JSON formatter (up to x2 compared to pretty printing with [Serde JSON](https://github.com/serde-rs/json)).
/// This formatter parses and formats JSON input byte by byte and do not require pre UTF-8 validation.
/// UTF-8 validation is done in-place, on the fly, while parsing strings. This implementation try to not allocate
/// anything. It does not try to normalise, remove unnecessary escaping, it just formats the actual input
/// with spaces, newlines and (optionally) color.
///
/// This formatter supports writing to a [`io::Write`] instance (file, standard output), or to a [`fmt::Write`]
/// buffer (string etc...). If the formatting fails, the write buffer may contain some unwanted data
/// from the already read bytes. It's up to the caller to deal with this kind of failure (clear a buffer
/// for instance etc...).
pub struct Formatter<'input> {
    /// The JSON input bytes to prettify.
    input: &'input [u8],
    /// Cursor position in byte offset (starting at 0)
    pos: BytePos,
    /// Current indentation level (this is maxed by [`MAX_INDENT_LEVEL`])
    level: usize,
    /// Use color with ANSI escape code when prettifying.
    color: Color,
}

/// The maximum indentation level supported before errors.
const MAX_INDENT_LEVEL: usize = 100;

/// A byte position in a bytes stream (0-based index).
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct BytePos(usize);

/// Potential errors raised during formatting.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FormatError {
    /// Unexpected end of file.
    Eof,
    /// Invalid byte at this position.
    InvalidByte(u8, BytePos),
    /// The next bytes are not a valid UTF-8 sequence.
    InvalidUtf8([u8; 4], usize, BytePos),
    /// Invalid escaped byte at this position.
    InvalidEscape(u8, BytePos),
    /// The maximum indent level has been reached.
    MaxIndentLevel(usize, BytePos),
    /// Write error occuring when formatting to an [`io::Write`] (file, standard output etc...)
    Io(io::ErrorKind),
    /// Write error occuring when formatting to an [`fmt::Write`] (string, etc...)
    Fmt(fmt::Error),
}

impl fmt::Display for FormatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn debug_str_u8(b: u8) -> String {
            match char::from(b) {
                c if c.is_ascii_graphic() || c == ' ' => {
                    format!("0x{b:02x?}/'{c}'")
                }
                _ => format!("0x{b:02x?}"),
            }
        }

        match self {
            FormatError::Eof => write!(f, "unexpected end of file"),
            FormatError::InvalidByte(byte, pos) => {
                let byte = debug_str_u8(*byte);
                write!(f, "invalid byte {byte} at offset {}", pos.0)
            }
            FormatError::InvalidUtf8(bytes, len, pos) => {
                let hex = bytes
                    .iter()
                    .take(*len)
                    .map(|b| format!("0x{:02x}", b))
                    .collect::<Vec<_>>()
                    .join(" ");
                write!(f, "invalid UTF-8 {} bytes {hex} at offset {}", len, pos.0)
            }
            FormatError::InvalidEscape(byte, pos) => {
                let byte = debug_str_u8(*byte);
                write!(f, "invalid escaped byte {byte} at offset {}", pos.0)
            }
            FormatError::MaxIndentLevel(level, pos) => {
                write!(f, "maximum indent level {} at offset {}", level, pos.0)
            }
            FormatError::Io(error) => write!(f, "error writing {error}"),
            FormatError::Fmt(error) => write!(f, "error writing {error}"),
        }
    }
}

impl From<io::Error> for FormatError {
    fn from(e: io::Error) -> Self {
        FormatError::Io(e.kind())
    }
}

impl From<fmt::Error> for FormatError {
    fn from(e: fmt::Error) -> Self {
        FormatError::Fmt(e)
    }
}

/// Whether we prettify JSON with [ANSI escape codes](https://en.wikipedia.org/wiki/ANSI_escape_code)
/// or not.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Color {
    NoColor,
    Ansi,
}

/// Is the current string token being processed semantically a "key" or a "value". The color
/// used for prettifying depends on it.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum StringMode {
    Key,
    Value,
}

type FormatResult<T> = Result<T, FormatError>;

/// Helpers method to prettify to an [`io::Write`] (File, standard output etc...)
pub fn format(input: &[u8], color: Color, out: &mut impl io::Write) -> FormatResult<()> {
    let mut formatter = Formatter::new(input, color);
    formatter.format(out)
}

impl<'input> Formatter<'input> {
    /// Creates a new formater, with JSON `input` bytes to format and colorize.
    pub fn new(input: &'input [u8], color: Color) -> Self {
        Formatter {
            input,
            pos: BytePos(0),
            level: 0,
            color,
        }
    }

    /// Reads the next byte and advances the read position.
    #[inline]
    fn next_byte(&mut self) -> Option<u8> {
        let b = self.peek_byte()?;
        self.pos.0 += 1;
        Some(b)
    }

    /// Peeks the next byte without advancing the read position.
    #[inline]
    fn peek_byte(&mut self) -> Option<u8> {
        self.input.get(self.pos.0).copied()
    }

    /// Reads the next byte, advances the read position and check the read value.
    #[inline]
    fn expect_byte(&mut self, expected: u8) -> FormatResult<()> {
        match self.next_byte() {
            Some(b) if b == expected => Ok(()),
            Some(b) => Err(FormatError::InvalidByte(b, BytePos(self.pos.0 - 1))),
            None => Err(FormatError::Eof),
        }
    }

    /// Increments the indentation level.
    fn inc_level(&mut self) -> FormatResult<()> {
        if self.level >= MAX_INDENT_LEVEL {
            return Err(FormatError::MaxIndentLevel(self.level, self.pos));
        }
        self.level += 1;
        Ok(())
    }

    /// Decrements the indentation level.
    fn dec_level(&mut self) {
        self.level -= 1;
    }

    /// Formats and colorize the JSON input bytes.
    pub fn format(&mut self, out: &mut impl io::Write) -> FormatResult<()> {
        self.skip_start_bom();

        self.skip_whitespace();
        self.parse_value(out)?;
        self.skip_whitespace();

        // End the prettified output with a trailing newline.
        self.write_ln(out)?;

        // Have we completely consumed our payload?
        if let Some(b) = self.peek_byte() {
            Err(FormatError::InvalidByte(b, self.pos))
        } else {
            Ok(())
        }
    }

    /// Skips BOM (Byte Order Mark) at the start of the read buffer.
    fn skip_start_bom(&mut self) {
        debug_assert!(self.pos.0 == 0);
        if self.input.len() < 3 {
            return;
        }
        if self.input[0] == 0xEF && self.input[1] == 0xBB && self.input[2] == 0xBF {
            self.pos.0 = 3;
        }
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.peek_byte(), Some(b' ' | b'\n' | b'\r' | b'\t')) {
            self.pos.0 += 1;
        }
    }

    /// Processes a JSON value.
    fn parse_value(&mut self, out: &mut impl io::Write) -> FormatResult<()> {
        // From <https://datatracker.ietf.org/doc/html/rfc7159#section-3>:
        //
        // value = false / null / true / object / array / number / string
        // false = %x66.61.6c.73.65   ; false
        // null  = %x6e.75.6c.6c      ; null
        // true  = %x74.72.75.65      ; true
        match self.peek_byte() {
            Some(b'"') => self.parse_string(out, StringMode::Value),
            Some(b'-' | b'0'..=b'9') => self.parse_number(out),
            Some(b'{') => self.parse_object(out),
            Some(b'[') => self.parse_array(out),
            Some(b't') => self.parse_true(out),
            Some(b'f') => self.parse_false(out),
            Some(b'n') => self.parse_null(out),
            Some(b) => Err(FormatError::InvalidByte(b, self.pos)),
            None => Err(FormatError::Eof),
        }
    }

    /// Processes a JSON object.
    fn parse_object(&mut self, out: &mut impl io::Write) -> FormatResult<()> {
        // From <https://datatracker.ietf.org/doc/html/rfc7159#section-4>:
        // object = begin-object [ member *( value-separator member ) ]
        // end-object
        // member = string name-separator value
        self.expect_byte(b'{')?;

        // For empty objects, we keep a short compact form:
        self.skip_whitespace();
        if self.peek_byte() == Some(b'}') {
            self.next_byte();
            self.write_empty_obj(out)?;
            return Ok(());
        }

        // Now, we have a non-empty object.
        self.write_begin_obj(out)?;
        self.inc_level()?;

        let mut first = true;
        loop {
            self.skip_whitespace();
            if self.peek_byte() == Some(b'}') {
                self.next_byte();
                self.dec_level();
                self.write_ln(out)?;
                self.write_indent(out)?;
                self.write_end_obj(out)?;
                return Ok(());
            }

            if first {
                first = false;
            } else {
                self.expect_byte(b',')?;
                self.skip_whitespace();
                self.write_value_sep(out)?;
            }

            // Parse key
            self.write_indent(out)?;
            self.parse_string(out, StringMode::Key)?;

            // Parse colon
            self.skip_whitespace();
            self.expect_byte(b':')?;
            self.write_name_sep(out)?;

            // Parse value
            self.skip_whitespace();
            self.parse_value(out)?;
        }
    }

    /// Processes a JSON array.
    fn parse_array(&mut self, out: &mut impl io::Write) -> FormatResult<()> {
        // From <https://datatracker.ietf.org/doc/html/rfc7159#section-4>:
        // array = begin-array [ value *( value-separator value ) ] end-array
        self.expect_byte(b'[')?;

        // For empty arrays, we keep a short compact form:
        self.skip_whitespace();
        if self.peek_byte() == Some(b']') {
            self.next_byte();
            self.write_empty_arr(out)?;
            return Ok(());
        }

        // Now, we have a non-empty array.
        self.write_begin_arr(out)?;
        self.inc_level()?;

        let mut first = true;
        loop {
            self.skip_whitespace();
            if self.peek_byte() == Some(b']') {
                self.next_byte();
                self.dec_level();
                self.write_ln(out)?;
                self.write_indent(out)?;
                self.write_end_arr(out)?;
                return Ok(());
            }

            if first {
                first = false;
            } else {
                self.expect_byte(b',')?;
                self.skip_whitespace();
                self.write_value_sep(out)?;
            }

            self.write_indent(out)?;
            self.parse_value(out)?;
        }
    }

    /// Processes a JSON string. The string is not normalized, escapes are preserved and the string
    /// bytes are validated on-the-fly to be UTF-8 valid.
    fn parse_string(&mut self, out: &mut impl io::Write, mode: StringMode) -> FormatResult<()> {
        // From <https://datatracker.ietf.org/doc/html/rfc7159#section-8>

        let start = self.pos;
        self.expect_byte(b'"')?;

        while let Some(b) = self.peek_byte() {
            match b {
                b'"' => {
                    self.next_byte();

                    // Flush plain segment before exit.
                    let string = &self.input[start.0..self.pos.0];
                    match mode {
                        StringMode::Key => self.write_key(string, out)?,
                        StringMode::Value => self.write_value(string, out)?,
                    };
                    return Ok(());
                }
                // Escaping
                b'\\' => {
                    self.next_byte();
                    match self.next_byte() {
                        Some(b'"' | b'\\' | b'/' | b'b' | b'f' | b'n' | b'r' | b't') => {}
                        Some(b'u') => {
                            for _ in 0..4 {
                                let hex = self.next_byte().ok_or(FormatError::Eof)?;
                                if !(hex as char).is_ascii_hexdigit() {
                                    return Err(FormatError::InvalidByte(
                                        hex,
                                        BytePos(self.pos.0 - 1),
                                    ));
                                }
                            }
                        }
                        Some(b) => return Err(FormatError::InvalidEscape(b, self.pos)),
                        None => return Err(FormatError::Eof),
                    }
                }
                0x00..=0x1F => return Err(FormatError::InvalidByte(b, self.pos)),
                _ => {
                    // Decode valid UTF-8 char
                    self.next_utf8_char()?;
                }
            }
        }
        Err(FormatError::Eof)
    }

    /// Processes a `true` literal.
    fn parse_true(&mut self, out: &mut impl io::Write) -> FormatResult<()> {
        for &b in b"true" {
            self.expect_byte(b)?;
        }
        self.write_true(out)?;
        Ok(())
    }

    /// Processes a `false` literal.
    fn parse_false(&mut self, out: &mut impl io::Write) -> FormatResult<()> {
        for &b in b"false" {
            self.expect_byte(b)?;
        }
        self.write_false(out)?;
        Ok(())
    }

    /// Processes a `null` literal.
    fn parse_null(&mut self, out: &mut impl io::Write) -> FormatResult<()> {
        for &b in b"null" {
            self.expect_byte(b)?;
        }
        self.write_null(out)?;
        Ok(())
    }

    /// Processes a JSON number.
    fn parse_number(&mut self, out: &mut impl io::Write) -> FormatResult<()> {
        // From the spec <https://datatracker.ietf.org/doc/html/rfc7159#section-6>:
        //
        // number = [ minus ] int [ frac ] [ exp ]
        // decimal-point = %x2E       ; .
        // digit1-9 = %x31-39         ; 1-9
        // e = %x65 / %x45            ; e E
        // exp = e [ minus / plus ] 1*DIGIT
        // frac = decimal-point 1*DIGIT
        // int = zero / ( digit1-9 *DIGIT )
        // minus = %x2D               ; -
        // plus = %x2B                ; +
        // zero = %x30                ; 0

        let start = self.pos;

        if self.peek_byte() == Some(b'-') {
            self.next_byte();
        }

        self.parse_integer()?;
        self.parse_fraction()?;
        self.parse_exponent()?;

        // Finally, write numbers
        let digits = &self.input[start.0..self.pos.0];
        self.write_number(digits, out)?;

        Ok(())
    }

    /// Processes the integer part of a number.
    fn parse_integer(&mut self) -> FormatResult<()> {
        match self.peek_byte() {
            Some(b'0') => {
                self.next_byte();
                Ok(())
            }
            Some(b'1'..=b'9') => {
                self.next_byte();
                // 0 or more digits
                while let Some(b'0'..=b'9') = self.peek_byte() {
                    self.next_byte();
                }
                Ok(())
            }
            Some(b) => Err(FormatError::InvalidByte(b, self.pos)),
            None => Err(FormatError::Eof),
        }
    }

    /// Processes the fractional part of a number.
    fn parse_fraction(&mut self) -> FormatResult<()> {
        if self.peek_byte() == Some(b'.') {
            self.next_byte();
            // 1 or more digits
            match self.peek_byte() {
                Some(b'0'..=b'9') => {
                    self.next_byte();
                    while let Some(b'0'..=b'9') = self.peek_byte() {
                        self.next_byte();
                    }
                    Ok(())
                }
                Some(b) => Err(FormatError::InvalidByte(b, self.pos)),
                None => Err(FormatError::Eof),
            }?;
        }
        Ok(())
    }

    /// Processes the exponent part of a number.
    fn parse_exponent(&mut self) -> FormatResult<()> {
        match self.peek_byte() {
            Some(b'e' | b'E') => {
                self.next_byte();
                if let Some(b'+' | b'-') = self.peek_byte() {
                    self.next_byte();
                }
                match self.peek_byte() {
                    Some(b'0'..=b'9') => {
                        self.next_byte();
                        while let Some(b'0'..=b'9') = self.peek_byte() {
                            self.next_byte();
                        }
                        Ok(())
                    }
                    Some(b) => Err(FormatError::InvalidByte(b, self.pos)),
                    None => Err(FormatError::Eof),
                }
            }
            _ => Ok(()),
        }
    }

    /// Read and advances to the next UTF-8 char (may advance 1 to 4 bytes).
    /// The code check for UTF-8 validity, reference is from <https://en.wikipedia.org/wiki/UTF-8>
    /// Bytes bounds values and logic are extracted from this [table](https://en.wikipedia.org/wiki/UTF-8#Byte_map):
    ///
    /// |   | 0   | 1   | 2   | 3   | 4   | 5   | 6   | 7   | 8   | 9   | A   | B   | C   | D   | E   | F   |
    /// |---|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
    /// | 0 | NUL | SOH | STX | ETX | EOT | ENQ | ACK | BEL | BS  | HT  | LF  | VT  | FF  | CR  | SO  | SI  |
    /// | 1 | DLE | DC1 | DC2 | DC3 | DC4 | NAK | SYN | ETB | CAN | EM  | SUB | ESC | FS  | GS  | RS  | US  |
    /// | 2 | SP  | !   | "   | #   | $   | %   | &   | '   | (   | )   | *   | +   | ,   | -   | .   | /   |
    /// | 3 | 0   | 1   | 2   | 3   | 4   | 5   | 6   | 7   | 8   | 9   | :   | ;   | <   | =   | >   | ?   |
    /// | 4 | @   | A   | B   | C   | D   | E   | F   | G   | H   | I   | J   | K   | L   | M   | N   | O   |
    /// | 5 | P   | Q   | R   | S   | T   | U   | V   | W   | X   | Y   | Z   | [   | \   | ]   | ^   | _   |
    /// | 6 | `   | a   | b   | c   | d   | e   | f   | g   | h   | i   | j   | k   | l   | m   | n   | o   |
    /// | 7 | p   | q   | r   | s   | t   | u   | v   | w   | x   | y   | z   | {   | \|  | }   | ~   | DEL |
    /// | 8 | con | con | con | con | con | con | con | con | con | con | con | con | con | con | con | con |
    /// | 9 | con | con | con | con | con | con | con | con | con | con | con | con | con | con | con | con |
    /// | A | con | con | con | con | con | con | con | con | con | con | con | con | con | con | con | con |
    /// | B | con | con | con | con | con | con | con | con | con | con | con | con | con | con | con | con |
    /// | C |  ▒  |  ▒  | 2   | 2   | 2   | 2   | 2   | 2   | 2   | 2   | 2   | 2   | 2   | 2   | 2   | 2   |
    /// | D | 2   | 2   | 2   | 2   | 2   | 2   | 2   | 2   | 2   | 2   | 2   | 2   | 2   | 2   | 2   | 2   |
    /// | E | 3   | 3   | 3   | 3   | 3   | 3   | 3   | 3   | 3   | 3   | 3   | 3   | 3   | 3   | 3   | 3   |
    /// | F | 4   | 4   | 4   | 4   | 4   |  ▒  |  ▒  |  ▒  |  ▒  |  ▒  |  ▒  |  ▒  |  ▒  |  ▒  |  ▒  |  ▒  |
    ///
    /// - con => Continuation byte
    /// - 2   => First byte of 2-byte sequence
    /// - 3   => First byte of 3-byte sequence
    /// - 4   => First byte of 4-byte sequence
    /// - ▒   => not used
    fn next_utf8_char(&mut self) -> FormatResult<()> {
        #[inline(always)]
        fn cont(b: u8) -> bool {
            (b & 0xC0) == 0x80
        }

        let start_pos = self.pos;

        // Case 1: Single-byte ASCII character (0xxxxxxx).
        let b1 = self.next_byte().ok_or(FormatError::Eof)?;
        if b1 < 0x80 {
            return Ok(());
        }

        // Case 2: Two-byte sequence (110xxxxx 10xxxxxx).
        let b2 = self.next_byte().ok_or(FormatError::Eof)?;
        if b1 < 0xE0 {
            return if (0xC2..=0xDF).contains(&b1) && cont(b2) {
                Ok(())
            } else {
                Err(FormatError::InvalidUtf8([b1, b2, 0, 0], 2, start_pos))
            };
        }

        // Case 3: Three-byte sequence (1110xxxx 10xxxxxx 10xxxxxx).
        let b3 = self.next_byte().ok_or(FormatError::Eof)?;
        if b1 < 0xF0 {
            return if match b1 {
                // See <https://en.wikipedia.org/wiki/UTF-8#Error_handling>
                // Overlong encodings:
                // > An overlong encoding (0xE0 followed by less than 0xA0, ...)
                0xE0 => (0xA0..=0xBF).contains(&b2) && cont(b3),
                // Can't be UTF-16 surrogates:
                // > A 3-byte sequence that decodes to a UTF-16 surrogate U+0xD800–0xDFFF (0xED followed by 0xA0 or greater)
                0xED => (0x80..=0x9F).contains(&b2) && cont(b3),
                // General case: 2 continuation bytes
                0xE1..=0xEC | 0xEE..=0xEF => cont(b2) && cont(b3),
                _ => false,
            } {
                Ok(())
            } else {
                Err(FormatError::InvalidUtf8([b1, b2, b3, 0], 3, self.pos))
            };
        }

        // Case 4: Four-byte sequence (11110xxx 10xxxxxx 10xxxxxx 10xxxxxx).
        let b4 = self.next_byte().ok_or(FormatError::Eof)?;
        if match b1 {
            // See <https://en.wikipedia.org/wiki/UTF-8#Error_handling>
            // Overlong encodings:
            // > An overlong encoding (..., or 0xF0 followed by less than 0x90)
            0xF0 => (0x90..=0xBF).contains(&b2) && cont(b3) && cont(b4),
            // Limit to code point 0x10FFFF
            // > A 4-byte sequence that decodes to a value greater than U+10FFFF (0xF4 followed by 0x90 or greater)
            0xF4 => (0x80..=0x8F).contains(&b2) && cont(b3) && cont(b4),
            // General case: 3 continuation bytes
            0xF1..=0xF3 => cont(b2) && cont(b3) && cont(b4),
            _ => false,
        } {
            Ok(())
        } else {
            Err(FormatError::InvalidUtf8([b1, b2, b3, b4], 4, self.pos))
        }
    }
}

const SPACES: &[u8] = b"                                                                 ";

/// Methods to print on a [Write], with color, or not.
impl<'input> Formatter<'input> {
    fn write_indent(&self, out: &mut impl io::Write) -> Result<(), io::Error> {
        let n = self.level * 2;
        let full_chunks = n / SPACES.len();
        let remainder = n % SPACES.len();
        for _ in 0..full_chunks {
            out.write_all(SPACES)?;
        }
        out.write_all(&SPACES[..remainder])?;
        Ok(())
    }

    #[inline]
    fn write_ln(&self, out: &mut impl io::Write) -> Result<(), io::Error> {
        out.write_all(b"\n")
    }

    #[inline]
    fn write_empty_obj(&self, out: &mut impl io::Write) -> Result<(), io::Error> {
        if self.color == Color::Ansi {
            out.write_all(b"\x1b[1;39m{}\x1b[0m")
        } else {
            out.write_all(b"{}")
        }
    }

    #[inline]
    fn write_begin_obj(&self, out: &mut impl io::Write) -> Result<(), io::Error> {
        if self.color == Color::Ansi {
            out.write_all(b"\x1b[1;39m{\x1b[0m\n")
        } else {
            out.write_all(b"{\n")
        }
    }

    #[inline]
    fn write_end_obj(&self, out: &mut impl io::Write) -> Result<(), io::Error> {
        if self.color == Color::Ansi {
            out.write_all(b"\x1b[1;39m}\x1b[0m")
        } else {
            out.write_all(b"}")
        }
    }

    #[inline]
    fn write_value_sep(&self, out: &mut impl io::Write) -> Result<(), io::Error> {
        if self.color == Color::Ansi {
            out.write_all(b"\x1b[1;39m,\x1b[0m\n")
        } else {
            out.write_all(b",\n")
        }
    }

    #[inline]
    fn write_name_sep(&self, out: &mut impl io::Write) -> Result<(), io::Error> {
        if self.color == Color::Ansi {
            out.write_all(b"\x1b[1;39m:\x1b[0m ")
        } else {
            out.write_all(b": ")
        }
    }

    #[inline]
    fn write_empty_arr(&self, out: &mut impl io::Write) -> Result<(), io::Error> {
        if self.color == Color::Ansi {
            out.write_all(b"\x1b[1;39m[]\x1b[0m")
        } else {
            out.write_all(b"[]")
        }
    }

    #[inline]
    fn write_begin_arr(&self, out: &mut impl io::Write) -> Result<(), io::Error> {
        if self.color == Color::Ansi {
            out.write_all(b"\x1b[1;39m[\x1b[0m\n")
        } else {
            out.write_all(b"[\n")
        }
    }

    #[inline]
    fn write_end_arr(&self, out: &mut impl io::Write) -> Result<(), io::Error> {
        if self.color == Color::Ansi {
            out.write_all(b"\x1b[1;39m]\x1b[0m")
        } else {
            out.write_all(b"]")
        }
    }

    #[inline]
    fn write_key(&self, s: &[u8], out: &mut impl io::Write) -> Result<(), io::Error> {
        if self.color == Color::Ansi {
            out.write_all(b"\x1b[1;34m")?;
            out.write_all(s)?;
            out.write_all(b"\x1b[0m")
        } else {
            out.write_all(s)
        }
    }

    #[inline]
    fn write_value(&self, s: &[u8], out: &mut impl io::Write) -> Result<(), io::Error> {
        if self.color == Color::Ansi {
            out.write_all(b"\x1b[0;32m")?;
            out.write_all(s)?;
            out.write_all(b"\x1b[0m")
        } else {
            out.write_all(s)
        }
    }

    #[inline]
    fn write_true(&self, out: &mut impl io::Write) -> Result<(), io::Error> {
        if self.color == Color::Ansi {
            out.write_all(b"\x1b[0;33mtrue\x1b[0m")
        } else {
            out.write_all(b"true")
        }
    }

    #[inline]
    fn write_false(&self, out: &mut impl io::Write) -> Result<(), io::Error> {
        if self.color == Color::Ansi {
            out.write_all(b"\x1b[0;33mfalse\x1b[0m")
        } else {
            out.write_all(b"false")
        }
    }

    #[inline]
    fn write_null(&self, out: &mut impl io::Write) -> Result<(), io::Error> {
        if self.color == Color::Ansi {
            out.write_all(b"\x1b[0;35mnull\x1b[0m")
        } else {
            out.write_all(b"null")
        }
    }

    #[inline]
    fn write_number(&self, s: &[u8], out: &mut impl io::Write) -> Result<(), io::Error> {
        if self.color == Color::Ansi {
            out.write_all(b"\x1b[0;36m")?;
            out.write_all(s)?;
            out.write_all(b"\x1b[0m")
        } else {
            out.write_all(s)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helpers method to prettify to an [`fmt::Write`] (String etc...)
    fn format_fmt(input: &[u8], color: Color, out: &mut impl fmt::Write) -> FormatResult<()> {
        let mut formatter = Formatter::new(input, color);
        let mut bytes = Vec::new();
        formatter.format(&mut bytes)?;
        // If format is successful, we are sure that the bytes are UTF-8 valid.
        let str = std::str::from_utf8(&bytes).unwrap();
        out.write_str(str)?;
        Ok(())
    }

    #[test]
    fn parse_number_ok() {
        let datas = [
            // Parse some integers
            ("1234xxxx", "1234"),
            ("42", "42"),
            (
                "1233456787766677889778998789988",
                "1233456787766677889778998789988",
            ),
            ("0000", "0"),
            ("0", "0"),
            ("-0", "-0"),
            ("012345", "0"),
            ("0abcdef", "0"),
            ("-10256", "-10256"),
            ("-012344", "-0"),
            // Parse real (with fraction)
            ("1.000", "1.000"),
            ("1.7b", "1.7"),
        ];
        for (input, expected) in datas {
            let mut formatter = Formatter::new(input.as_bytes(), Color::NoColor);
            let mut out = Vec::new();
            formatter.parse_number(&mut out).unwrap();
            assert_eq!(String::from_utf8(out).unwrap(), expected);
        }
    }

    #[test]
    fn parse_number_failed() {
        let datas = ["1.", "78980.a", "abc"];
        for input in datas {
            let mut formatter = Formatter::new(input.as_bytes(), Color::NoColor);
            let mut out = Vec::new();
            let result = formatter.parse_number(&mut out);
            assert!(result.is_err());
        }
    }

    fn assert_against_std(bytes: &[u8], len: usize) {
        // We pass the full buffer to the parser, with some trailing bytes
        let mut formatter = Formatter::new(bytes, Color::NoColor);
        let ret = formatter.next_utf8_char();

        // We test against a buffer without trailing
        match std::str::from_utf8(&bytes[..len]) {
            Ok(str) => {
                assert!(ret.is_ok());
                assert_eq!(formatter.pos.0, len);
                let out = str::from_utf8(&formatter.input[0..formatter.pos.0]).unwrap();
                assert_eq!(out, str);
            }
            Err(_) => {
                assert!(ret.is_err());
            }
        }
    }

    #[test]
    fn try_read_one_byte_to_utf8() {
        // Iterate through all 1-byte UTF-8 bytes, even invalid
        for b in 0x00..=0xFF {
            let bytes = [b, b'x', b'x', b'x'];
            assert_against_std(&bytes, 1);
        }
    }

    #[test]
    fn try_read_two_bytes_to_utf8() {
        // Iterate through all UTF-8 2-bytes: C0..=DF 80..=BF
        // It may contains invalid ones (overlong for instance).
        for b1 in 0xC0..=0xDF {
            for b2 in 0x80..=0xBF {
                let bytes = [b1, b2, b'x', b'x', b'x'];
                assert_against_std(&bytes, 2);
            }
        }
    }

    #[test]
    fn try_read_three_bytes_to_utf8() {
        // Iterate through all UTF-8 3-bytes: E0..=EF 80..=BF 80..=BF
        // It may contains invalid ones (overlong for instance).
        for b1 in 0xF0..=0xF7 {
            for b2 in 0x80..=0xBF {
                for b3 in 0x80..=0xBF {
                    let bytes = [b1, b2, b3, b'x', b'x', b'x'];
                    assert_against_std(&bytes, 3);
                }
            }
        }
    }

    #[test]
    fn try_read_four_bytes_to_utf8() {
        // Iterate through all UTF-8 4-bytes: F0..=F7 80..=BF 80..=BF 80..=BF
        // It may contains invalid ones (overlong for instance).
        for b1 in 0xF0..=0xF7 {
            for b2 in 0x80..=0xBF {
                for b3 in 0x80..=0xBF {
                    for b4 in 0x80..=0xBF {
                        let bytes = [b1, b2, b3, b4, b'x', b'x', b'x'];
                        assert_against_std(&bytes, 4);
                    }
                }
            }
        }
    }

    #[test]
    fn format_valid_json() {
        struct TestData {
            input: &'static str,
            expected: &'static str,
        }

        let datas = [
            TestData {
                input: r#"{"strings":{"english":"Hello, world!","chinese":"你好，世界","japanese":"こんにちは世界","korean":"안녕하세요 세계","arabic":"مرحبا بالعالم","hindi":"नमस्ते दुनिया","russian":"Привет, мир","greek":"Γειά σου Κόσμε","hebrew":"שלום עולם","accented":"Curaçao, naïve, façade, jalapeño"},"numbers":{"zero":0,"positive_int":42,"negative_int":-42,"large_int":1234567890123456789,"small_float":0.000123,"negative_float":-3.14159,"large_float":1.7976931348623157e308,"smallest_float":5e-324,"sci_notation_positive":6.022e23,"sci_notation_negative":-2.99792458e8},"booleans":{"isActive":true,"isDeleted":false},"emojis":{"happy":"😀","sad":"😢","fire":"🔥","rocket":"🚀","earth":"🌍","heart":"❤️","multi":"👩‍💻🧑🏽‍🚀👨‍👩‍👧‍👦"},"nothing":null}"#,
                expected: r#"{
  "strings": {
    "english": "Hello, world!",
    "chinese": "你好，世界",
    "japanese": "こんにちは世界",
    "korean": "안녕하세요 세계",
    "arabic": "مرحبا بالعالم",
    "hindi": "नमस्ते दुनिया",
    "russian": "Привет, мир",
    "greek": "Γειά σου Κόσμε",
    "hebrew": "שלום עולם",
    "accented": "Curaçao, naïve, façade, jalapeño"
  },
  "numbers": {
    "zero": 0,
    "positive_int": 42,
    "negative_int": -42,
    "large_int": 1234567890123456789,
    "small_float": 0.000123,
    "negative_float": -3.14159,
    "large_float": 1.7976931348623157e308,
    "smallest_float": 5e-324,
    "sci_notation_positive": 6.022e23,
    "sci_notation_negative": -2.99792458e8
  },
  "booleans": {
    "isActive": true,
    "isDeleted": false
  },
  "emojis": {
    "happy": "😀",
    "sad": "😢",
    "fire": "🔥",
    "rocket": "🚀",
    "earth": "🌍",
    "heart": "❤️",
    "multi": "👩‍💻🧑🏽‍🚀👨‍👩‍👧‍👦"
  },
  "nothing": null
}
"#,
            },
            // From Go Standard library <https://github.com/golang/go/blob/master/src/encoding/json/jsontext/value_test.go>
            // Primitives
            TestData {
                input: r#"{
		"numbers": [333333333.33333329, 1E30, 4.50,
					2e-3, 0.000000000000000000000000001, -0],
		"string": "\u20ac$\u000F\u000aA'\u0042\u0022\u005c\\\"\/",
		"literals": [null, true, false]
	}"#,
                expected: r#"{
  "numbers": [
    333333333.33333329,
    1E30,
    4.50,
    2e-3,
    0.000000000000000000000000001,
    -0
  ],
  "string": "\u20ac$\u000F\u000aA'\u0042\u0022\u005c\\\"\/",
  "literals": [
    null,
    true,
    false
  ]
}
"#,
            },
            TestData {
                input: r#"{
		"\u20ac": "Euro Sign",
		"\r": "Carriage Return",
		"\ufb33": "Hebrew Letter Dalet With Dagesh",
		"1": "One",
		"\ud83d\ude00": "Emoji: Grinning Face",
		"\u0080": "Control",
		"\u00f6": "Latin Small Letter O With Diaeresis"
	}"#,
                expected: r#"{
  "\u20ac": "Euro Sign",
  "\r": "Carriage Return",
  "\ufb33": "Hebrew Letter Dalet With Dagesh",
  "1": "One",
  "\ud83d\ude00": "Emoji: Grinning Face",
  "\u0080": "Control",
  "\u00f6": "Latin Small Letter O With Diaeresis"
}
"#,
            },
            // LargeIntegers
            TestData {
                input: " [ -9223372036854775808 , 9223372036854775807 ] ",
                expected: r#"[
  -9223372036854775808,
  9223372036854775807
]
"#,
            },
            // Duplicates
            TestData {
                input: r#" { "0" : 0 , "1" : 1 , "0" : 0 }"#,
                expected: r#"{
  "0": 0,
  "1": 1,
  "0": 0
}
"#,
            },
            // From jq "torture" tests <https://github.com/jqlang/jq/blob/master/tests/torture/input0.json>
            TestData {
                input: "[0,1,[12,22,[34,[45,56],7]],[]]",
                expected: r#"[
  0,
  1,
  [
    12,
    22,
    [
      34,
      [
        45,
        56
      ],
      7
    ]
  ],
  []
]
"#,
            },
            TestData {
                input: r#"{"a":[{"b":[]},{},[2]]}"#,
                expected: r#"{
  "a": [
    {
      "b": []
    },
    {},
    [
      2
    ]
  ]
}
"#,
            },
            TestData {
                input: " { }",
                expected: "{}\n",
            },
            TestData {
                input: r#"{"X":{},"Y":{},"X":{}} "#,
                expected: r#"{
  "X": {},
  "Y": {},
  "X": {}
}
"#,
            },
        ];

        for TestData { input, expected } in datas {
            let mut out = String::new();
            format_fmt(input.as_bytes(), Color::NoColor, &mut out).unwrap();
            assert_eq!(out, expected);
        }
    }

    // From Go Standard library <https://github.com/golang/go/blob/master/src/encoding/json/jsontext/decode_test.go>
    #[test]
    fn error_on_invalid() {
        struct TestData {
            name: &'static str,
            input: &'static [u8],
            expected_err: FormatError,
            expected_message: &'static str,
        }

        let datas = [
            TestData {
                name: "Invalid start",
                input: b" #",
                expected_err: FormatError::InvalidByte(35, BytePos(1)),
                expected_message: "invalid byte 0x23/'#' at offset 1",
            },
            TestData {
                name: "Extra comma",
                input: b" null , null ",
                expected_err: FormatError::InvalidByte(44, BytePos(6)),
                expected_message: "invalid byte 0x2c/',' at offset 6",
            },
            TestData {
                name: "Truncated null",
                input: b" nul",
                expected_err: FormatError::Eof,
                expected_message: "unexpected end of file",
            },
            TestData {
                name: "Invalid null",
                input: b"nulL",
                expected_err: FormatError::InvalidByte(76, BytePos(3)),
                expected_message: "invalid byte 0x4c/'L' at offset 3",
            },
            TestData {
                name: "Truncated false",
                input: b"fals",
                expected_err: FormatError::Eof,
                expected_message: "unexpected end of file",
            },
            TestData {
                name: "Invalid false",
                input: b"falsE",
                expected_err: FormatError::InvalidByte(69, BytePos(4)),
                expected_message: "invalid byte 0x45/'E' at offset 4",
            },
            TestData {
                name: "Truncated true",
                input: b"tru",
                expected_err: FormatError::Eof,
                expected_message: "unexpected end of file",
            },
            TestData {
                name: "Invalid true",
                input: b"truE",
                expected_err: FormatError::InvalidByte(69, BytePos(3)),
                expected_message: "invalid byte 0x45/'E' at offset 3",
            },
            TestData {
                name: "Invalid string",
                input: br#""start"#,
                expected_err: FormatError::Eof,
                expected_message: "unexpected end of file",
            },
            TestData {
                name: "Truncated string",
                input: br#""start"#,
                expected_err: FormatError::Eof,
                expected_message: "unexpected end of file",
            },
            TestData {
                name: "Invalid string",
                input: b"\"ok \x00",
                expected_err: FormatError::InvalidByte(0, BytePos(4)),
                expected_message: "invalid byte 0x00 at offset 4",
            },
            TestData {
                name: "Invalid UTF-8",
                input: b"\"living\xde\xad\xbe\xef\"",
                expected_err: FormatError::InvalidUtf8([0xbe, 0xef, 0, 0], 2, BytePos(9)),
                expected_message: "invalid UTF-8 2 bytes 0xbe 0xef at offset 9",
            },
            TestData {
                name: "Truncated number",
                input: b"0.",
                expected_err: FormatError::Eof,
                expected_message: "unexpected end of file",
            },
            TestData {
                name: "Invalid number",
                input: b"0.e",
                expected_err: FormatError::InvalidByte(101, BytePos(2)),
                expected_message: "invalid byte 0x65/'e' at offset 2",
            },
            TestData {
                name: "Truncated number after start",
                input: b"{",
                expected_err: FormatError::Eof,
                expected_message: "unexpected end of file",
            },
            TestData {
                name: "Truncated number after start",
                input: b"{",
                expected_err: FormatError::Eof,
                expected_message: "unexpected end of file",
            },
            TestData {
                name: "Truncated number after name",
                input: br#"{"0""#,
                expected_err: FormatError::Eof,
                expected_message: "unexpected end of file",
            },
            TestData {
                name: "Truncated number after colon",
                input: br#"{"0":"#,
                expected_err: FormatError::Eof,
                expected_message: "unexpected end of file",
            },
            TestData {
                name: "Truncated number after value",
                input: br#"{"0":0"#,
                expected_err: FormatError::Eof,
                expected_message: "unexpected end of file",
            },
            TestData {
                name: "Truncated number after comma",
                input: br#"{"0":0,"#,
                expected_err: FormatError::Eof,
                expected_message: "unexpected end of file",
            },
            TestData {
                name: "Invalid object missing colon",
                input: br#" { "fizz" "buzz" } "#,
                expected_err: FormatError::InvalidByte(34, BytePos(10)),
                expected_message: "invalid byte 0x22/'\"' at offset 10",
            },
            TestData {
                name: "Invalid object missing colon got comma",
                input: br#" { "fizz" , "buzz" } "#,
                expected_err: FormatError::InvalidByte(44, BytePos(10)),
                expected_message: "invalid byte 0x2c/',' at offset 10",
            },
            TestData {
                name: "Invalid object missing colon got hash",
                input: br#" { "fizz" # "buzz" } "#,
                expected_err: FormatError::InvalidByte(35, BytePos(10)),
                expected_message: "invalid byte 0x23/'#' at offset 10",
            },
            TestData {
                name: "Invalid object missing comma",
                input: br#" { "fizz" : "buzz" "gazz" } "#,
                expected_err: FormatError::InvalidByte(34, BytePos(19)),
                expected_message: "invalid byte 0x22/'\"' at offset 19",
            },
            TestData {
                name: "Invalid object missing comma got colon",
                input: br#" { "fizz" : "buzz" : "gazz" } "#,
                expected_err: FormatError::InvalidByte(58, BytePos(19)),
                expected_message: "invalid byte 0x3a/':' at offset 19",
            },
            TestData {
                name: "Invalid object missing comma got hash",
                input: br#" { "fizz" : "buzz" # "gazz" } "#,
                expected_err: FormatError::InvalidByte(35, BytePos(19)),
                expected_message: "invalid byte 0x23/'#' at offset 19",
            },
            TestData {
                name: "Invalid object extra comma after start",
                input: br#" { , } "#,
                expected_err: FormatError::InvalidByte(44, BytePos(3)),
                expected_message: "invalid byte 0x2c/',' at offset 3",
            },
            TestData {
                name: "Invalid object extra comma after value",
                input: br#" { "fizz" : "buzz" , } "#,
                expected_err: FormatError::InvalidByte(125, BytePos(21)),
                expected_message: "invalid byte 0x7d/'}' at offset 21",
            },
            TestData {
                name: "Invalid object invalid name got null",
                input: br#" { null : null } "#,
                expected_err: FormatError::InvalidByte(110, BytePos(3)),
                expected_message: "invalid byte 0x6e/'n' at offset 3",
            },
            TestData {
                name: "Invalid object invalid name got false",
                input: br#" { false : false } "#,
                expected_err: FormatError::InvalidByte(102, BytePos(3)),
                expected_message: "invalid byte 0x66/'f' at offset 3",
            },
            TestData {
                name: "Invalid object invalid name got true",
                input: br#" { true : true } "#,
                expected_err: FormatError::InvalidByte(116, BytePos(3)),
                expected_message: "invalid byte 0x74/'t' at offset 3",
            },
            TestData {
                name: "Invalid object invalid name got number",
                input: br#" { 0 : 0 } "#,
                expected_err: FormatError::InvalidByte(48, BytePos(3)),
                expected_message: "invalid byte 0x30/'0' at offset 3",
            },
            TestData {
                name: "Invalid object invalid name got object",
                input: br#" { {} : {} } "#,
                expected_err: FormatError::InvalidByte(123, BytePos(3)),
                expected_message: "invalid byte 0x7b/'{' at offset 3",
            },
            TestData {
                name: "Invalid object invalid name got array",
                input: br#" { [] : [] } "#,
                expected_err: FormatError::InvalidByte(91, BytePos(3)),
                expected_message: "invalid byte 0x5b/'[' at offset 3",
            },
            TestData {
                name: "Invalid object mismatching delim",
                input: br#" { ] "#,
                expected_err: FormatError::InvalidByte(93, BytePos(3)),
                expected_message: "invalid byte 0x5d/']' at offset 3",
            },
            TestData {
                name: "Invalid object mismatching delim",
                input: br#" { ] "#,
                expected_err: FormatError::InvalidByte(93, BytePos(3)),
                expected_message: "invalid byte 0x5d/']' at offset 3",
            },
            TestData {
                name: "Truncated array after start",
                input: b"[",
                expected_err: FormatError::Eof,
                expected_message: "unexpected end of file",
            },
            TestData {
                name: "Truncated array after value",
                input: b"[0",
                expected_err: FormatError::Eof,
                expected_message: "unexpected end of file",
            },
            TestData {
                name: "Truncated array after comma",
                input: b"[0,",
                expected_err: FormatError::Eof,
                expected_message: "unexpected end of file",
            },
            TestData {
                name: "Invalid array missing comma",
                input: br#"[ "fizz" "buzz" ] "#,
                expected_err: FormatError::InvalidByte(34, BytePos(9)),
                expected_message: "invalid byte 0x22/'\"' at offset 9",
            },
            TestData {
                name: "Invalid array mismatching delim",
                input: b" [ } ",
                expected_err: FormatError::InvalidByte(125, BytePos(3)),
                expected_message: "invalid byte 0x7d/'}' at offset 3",
            },
            TestData {
                name: "Invalid delim after top level",
                input: br#" "", "#,
                expected_err: FormatError::InvalidByte(44, BytePos(3)),
                expected_message: "invalid byte 0x2c/',' at offset 3",
            },
            TestData {
                name: "Invalid delim after begin object",
                input: b"{:",
                expected_err: FormatError::InvalidByte(58, BytePos(1)),
                expected_message: "invalid byte 0x3a/':' at offset 1",
            },
            TestData {
                name: "Invalid delim after object name",
                input: br#"{"","#,
                expected_err: FormatError::InvalidByte(44, BytePos(3)),
                expected_message: "invalid byte 0x2c/',' at offset 3",
            },
            TestData {
                name: "Valid delim after object name",
                input: br#"{"":"#,
                expected_err: FormatError::Eof,
                expected_message: "unexpected end of file",
            },
            TestData {
                name: "Invalid delim after object value",
                input: br#"{"":"":"#,
                expected_err: FormatError::InvalidByte(58, BytePos(6)),
                expected_message: "invalid byte 0x3a/':' at offset 6",
            },
            TestData {
                name: "Valid delim after object value",
                input: br#"{"":"","#,
                expected_err: FormatError::Eof,
                expected_message: "unexpected end of file",
            },
            TestData {
                name: "Invalid delim after begin array",
                input: b"[,",
                expected_err: FormatError::InvalidByte(44, BytePos(1)),
                expected_message: "invalid byte 0x2c/',' at offset 1",
            },
            TestData {
                name: "Invalid delim after array value",
                input: br#"["":"#,
                expected_err: FormatError::InvalidByte(58, BytePos(3)),
                expected_message: "invalid byte 0x3a/':' at offset 3",
            },
            TestData {
                name: "Valid delim after array value",
                input: br#"["","#,
                expected_err: FormatError::Eof,
                expected_message: "unexpected end of file",
            },
            TestData {
                name: "Error position",
                input: b"\"a\xff000\"",
                expected_err: FormatError::InvalidUtf8([255, 48, 48, 48], 4, BytePos(6)),
                expected_message: "invalid UTF-8 4 bytes 0xff 0x30 0x30 0x30 at offset 6",
            },
            TestData {
                name: "Error position /0",
                input: b" [ \"a\xff111\" ] ",
                expected_err: FormatError::InvalidUtf8([255, 49, 49, 49], 4, BytePos(9)),
                expected_message: "invalid UTF-8 4 bytes 0xff 0x31 0x31 0x31 at offset 9",
            },
            TestData {
                name: "Error position /1",
                input: b" [ \"a1\" , \"b\xff111\" ] ",
                expected_err: FormatError::InvalidUtf8([255, 49, 49, 49], 4, BytePos(16)),
                expected_message: "invalid UTF-8 4 bytes 0xff 0x31 0x31 0x31 at offset 16",
            },
            TestData {
                name: "Error position /0/0",
                input: b" [ [ \"a\xff222\" ] ] ",
                expected_err: FormatError::InvalidUtf8([255, 50, 50, 50], 4, BytePos(11)),
                expected_message: "invalid UTF-8 4 bytes 0xff 0x32 0x32 0x32 at offset 11",
            },
            TestData {
                name: "Error position /1/0",
                input: b" [ \"a1\" , [ \"a\xff222\" ] ] ",
                expected_err: FormatError::InvalidUtf8([255, 50, 50, 50], 4, BytePos(18)),
                expected_message: "invalid UTF-8 4 bytes 0xff 0x32 0x32 0x32 at offset 18",
            },
            TestData {
                name: "Error position /0/1",
                input: b" [ [ \"a2\" , \"b\xff222\" ] ] ",
                expected_err: FormatError::InvalidUtf8([255, 50, 50, 50], 4, BytePos(18)),
                expected_message: "invalid UTF-8 4 bytes 0xff 0x32 0x32 0x32 at offset 18",
            },
            TestData {
                name: "Error position /1/1",
                input: b" [ \"a1\" , [ \"a2\" , \"b\xff222\" ] ] ",
                expected_err: FormatError::InvalidUtf8([255, 50, 50, 50], 4, BytePos(25)),
                expected_message: "invalid UTF-8 4 bytes 0xff 0x32 0x32 0x32 at offset 25",
            },
            TestData {
                name: "Error position /a1-",
                input: b" { \"a\xff111\" : \"b1\" } ",
                expected_err: FormatError::InvalidUtf8([255, 49, 49, 49], 4, BytePos(9)),
                expected_message: "invalid UTF-8 4 bytes 0xff 0x31 0x31 0x31 at offset 9",
            },
            TestData {
                name: "Error position /a1",
                input: b" { \"a1\" : \"b\xff111\" } ",
                expected_err: FormatError::InvalidUtf8([255, 49, 49, 49], 4, BytePos(16)),
                expected_message: "invalid UTF-8 4 bytes 0xff 0x31 0x31 0x31 at offset 16",
            },
            TestData {
                name: "Error position /c1-",
                input: b" { \"a1\" : \"b1\" , \"c\xff111\" : \"d1\" } ",
                expected_err: FormatError::InvalidUtf8([255, 49, 49, 49], 4, BytePos(23)),
                expected_message: "invalid UTF-8 4 bytes 0xff 0x31 0x31 0x31 at offset 23",
            },
            TestData {
                name: "Error position /c1",
                input: b"{ \"a1\" : \"b1\" , \"c1\" : \"d\xff111\" } ",
                expected_err: FormatError::InvalidUtf8([255, 49, 49, 49], 4, BytePos(29)),
                expected_message: "invalid UTF-8 4 bytes 0xff 0x31 0x31 0x31 at offset 29",
            },
            TestData {
                name: "Error position /a1/a2-",
                input: b" { \"a1\" : { \"a\xff222\" : \"b2\" } } ",
                expected_err: FormatError::InvalidUtf8([255, 50, 50, 50], 4, BytePos(18)),
                expected_message: "invalid UTF-8 4 bytes 0xff 0x32 0x32 0x32 at offset 18",
            },
            TestData {
                name: "Error position /a1/a2",
                input: b" { \"a1\" : { \"a2\" : \"b\xff222\" } } ",
                expected_err: FormatError::InvalidUtf8([255, 50, 50, 50], 4, BytePos(25)),
                expected_message: "invalid UTF-8 4 bytes 0xff 0x32 0x32 0x32 at offset 25",
            },
            TestData {
                name: "Error position /a1/c2-",
                input: b" { \"a1\" : { \"a2\" : \"b2\" , \"c\xff222\" : \"d2\" } } ",
                expected_err: FormatError::InvalidUtf8([255, 50, 50, 50], 4, BytePos(32)),
                expected_message: "invalid UTF-8 4 bytes 0xff 0x32 0x32 0x32 at offset 32",
            },
            TestData {
                name: "Error position /a1/c2",
                input: b" { \"a1\" : { \"a2\" : \"b2\" , \"c2\" : \"d\xff222\" } } ",
                expected_err: FormatError::InvalidUtf8([255, 50, 50, 50], 4, BytePos(39)),
                expected_message: "invalid UTF-8 4 bytes 0xff 0x32 0x32 0x32 at offset 39",
            },
            TestData {
                name: "Error position /1/a2",
                input: b" [ \"a1\" , { \"a2\" : \"b\xff222\" } ] ",
                expected_err: FormatError::InvalidUtf8([255, 50, 50, 50], 4, BytePos(25)),
                expected_message: "invalid UTF-8 4 bytes 0xff 0x32 0x32 0x32 at offset 25",
            },
            TestData {
                name: "Error position /c1/1",
                input: b" { \"a1\" : \"b1\" , \"c1\" : [ \"a2\" , \"b\xff222\" ] } ",
                expected_err: FormatError::InvalidUtf8([255, 50, 50, 50], 4, BytePos(39)),
                expected_message: "invalid UTF-8 4 bytes 0xff 0x32 0x32 0x32 at offset 39",
            },
            TestData {
                name: "Error position /0/a1/1/c3/1",
                input: b" [ { \"a1\" : [ \"a2\" , { \"a3\" : \"b3\" , \"c3\" : [ \"a4\" , \"b\xff444\" ] } ] } ] ",
                expected_err: FormatError::InvalidUtf8([255, 52, 52, 52], 4, BytePos(59)),
                expected_message: "invalid UTF-8 4 bytes 0xff 0x34 0x34 0x34 at offset 59",
            },
        ];

        for TestData {
            name,
            input,
            expected_err,
            expected_message,
        } in datas
        {
            let mut out = String::new();
            let ret = format_fmt(input, Color::NoColor, &mut out);
            let err = ret.unwrap_err();
            assert_eq!(err, expected_err, "{name}");
            assert_eq!(err.to_string(), expected_message, "{name}");
        }
    }
}
