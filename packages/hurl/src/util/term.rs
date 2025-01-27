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
//! Wrapper on standard output/error.
use std::io;
#[cfg(target_family = "windows")]
use std::io::IsTerminal;
use std::io::Write;

/// The way to write on standard output and error: either immediate like `println!` macro,
/// or buffered in an internal buffer.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum WriteMode {
    /// Messages are printed immediately.
    Immediate,
    /// Messages are saved to an internal buffer, and can be retrieved with [`Stdout::buffer`] /
    /// [`Stderr::buffer`].
    Buffered,
}

/// Indirection for standard output.
///
/// Depending on `mode`, bytes are immediately printed to standard output, or buffered in an
/// internal buffer.
pub struct Stdout {
    /// Write mode of the standard output: immediate or saved to a buffer.
    mode: WriteMode,
    /// Internal buffer, filled when `mode` is [`WriteMode::Buffered`]
    buffer: Vec<u8>,
}

impl Stdout {
    /// Creates a new standard output, buffered or immediate depending on `mode`.
    pub fn new(mode: WriteMode) -> Self {
        Stdout {
            mode,
            buffer: Vec::new(),
        }
    }

    /// Attempts to write an entire buffer into standard output.
    pub fn write_all(&mut self, buf: &[u8]) -> Result<(), io::Error> {
        match self.mode {
            WriteMode::Immediate => write_stdout(buf),
            WriteMode::Buffered => self.buffer.write_all(buf),
        }
    }

    /// Returns the buffered standard output.
    pub fn buffer(&self) -> &[u8] {
        &self.buffer
    }
}

#[cfg(target_family = "unix")]
fn write_stdout(buf: &[u8]) -> Result<(), io::Error> {
    let mut handle = io::stdout().lock();
    handle.write_all(buf)?;
    Ok(())
}

#[cfg(target_family = "windows")]
fn write_stdout(buf: &[u8]) -> Result<(), io::Error> {
    // From <https://doc.rust-lang.org/std/io/struct.Stdout.html>:
    // > When operating in a console, the Windows implementation of this stream does not support
    // > non-UTF-8 byte sequences. Attempting to write bytes that are not valid UTF-8 will return
    // > an error.
    // As a workaround to prevent error, we convert the buffer to an UTF-8 string (with potential
    // bytes losses) before writing to the standard output of the Windows console.
    if io::stdout().is_terminal() {
        println!("{}", String::from_utf8_lossy(buf));
    } else {
        let mut handle = io::stdout().lock();
        handle.write_all(buf)?;
    }
    Ok(())
}

/// Indirection for standard error.
///
/// Depending on `mode`, messages are immediately printed to standard error, or buffered in an
/// internal buffer.
///
/// An optional `progress` string can be used to report temporary progress indication to the user.
/// It's always printed as the last lines of the standard error. When the standard error is created
/// with [`WriteMode::Buffered`], the progress is not saved in the internal buffer.
#[derive(Clone, Debug)]
pub struct Stderr {
    /// Write mode of the standard error: immediate or saved to a buffer.
    mode: WriteMode,
    /// Internal buffer, filled when `mode` is [`WriteMode::Buffered`]
    buffer: String,
    /// Progress bar: when not empty, it is always displayed at the end of the terminal.
    progress_bar: String,
}

impl Stderr {
    /// Creates a new standard error, buffered or immediate depending on `mode`.
    pub fn new(mode: WriteMode) -> Self {
        Stderr {
            mode,
            buffer: String::new(),
            progress_bar: String::new(),
        }
    }

    /// Returns the [`WriteMode`] of this logger.
    pub fn mode(&self) -> WriteMode {
        self.mode
    }

    /// Prints to the standard error, with a newline.
    pub fn eprintln(&mut self, message: &str) {
        match self.mode {
            WriteMode::Immediate => {
                let has_progress = !self.progress_bar.is_empty();
                if has_progress {
                    self.rewind_cursor();
                }
                eprintln!("{message}");
                if has_progress {
                    eprint!("{}", self.progress_bar);
                }
            }
            WriteMode::Buffered => {
                self.buffer.push_str(message);
                self.buffer.push('\n');
            }
        }
    }

    /// Prints to the standard error.
    pub fn eprint(&mut self, message: &str) {
        match self.mode {
            WriteMode::Immediate => {
                let has_progress = !self.progress_bar.is_empty();
                if has_progress {
                    self.rewind_cursor();
                }
                eprint!("{message}");
                if has_progress {
                    eprint!("{}", self.progress_bar);
                }
            }
            WriteMode::Buffered => {
                self.buffer.push_str(message);
            }
        }
    }

    /// Sets the progress bar (only in [`WriteMode::Immediate`] mode).
    pub fn set_progress_bar(&mut self, progress: &str) {
        match self.mode {
            WriteMode::Immediate => {
                self.progress_bar = progress.to_string();
                eprint!("{}", self.progress_bar);
            }
            WriteMode::Buffered => {}
        }
    }

    /// Clears the progress string (only in [`WriteMode::Immediate`] mode).
    pub fn clear_progress_bar(&mut self) {
        self.rewind_cursor();
        self.progress_bar.clear();
    }

    /// Returns the buffered standard error.
    pub fn buffer(&self) -> &str {
        &self.buffer
    }

    /// Set the buffered standard error.
    pub fn set_buffer(&mut self, buffer: String) {
        self.buffer = buffer;
    }

    /// Clears any progress and reset cursor terminal to the position of the last "real" message.
    fn rewind_cursor(&self) {
        if self.progress_bar.is_empty() {
            return;
        }
        match self.mode {
            WriteMode::Immediate => {
                // We count the number of new lines \n. We can't use the `String::lines()` because
                // it counts a line for a single carriage return. We don't want to go up for a
                // single carriage return.
                let lines = self.progress_bar.chars().filter(|c| *c == '\n').count();

                // We used the following ANSI codes:
                // - K: "EL - Erase in Line" sequence. It clears from the cursor to the end of line.
                // - 1A: "Cursor Up". Up to one line
                // <https://en.wikipedia.org/wiki/ANSI_escape_code#CSI_sequences>
                if lines > 0 {
                    (0..lines).for_each(|_| eprint!("\x1B[1A\x1B[K"));
                } else {
                    eprint!("\x1B[K");
                }
            }
            WriteMode::Buffered => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::util::term::{Stderr, Stdout, WriteMode};

    #[test]
    fn buffered_stdout() {
        let mut stdout = Stdout::new(WriteMode::Buffered);
        stdout.write_all(b"Hello").unwrap();
        stdout.write_all(b" ").unwrap();
        stdout.write_all(b"World!").unwrap();
        assert_eq!(stdout.buffer(), b"Hello World!");
    }

    #[test]
    fn buffered_stderr() {
        let mut stderr = Stderr::new(WriteMode::Buffered);
        stderr.eprintln("toto");
        stderr.set_progress_bar("some progress...\r");
        stderr.eprintln("tutu");

        assert_eq!(stderr.buffer(), "toto\ntutu\n");
    }
}
