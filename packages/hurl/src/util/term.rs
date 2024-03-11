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

/// Indirection for standard error.
///
/// Depending on `mode`, messages are immediately printed to standard error, or buffered
/// in an internal buffer.
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
    /// Progress string: when not empty, it is always displayed at the end of the terminal.
    progress: String,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum WriteMode {
    /// Messages are printed immediately.
    Immediate,
    /// Messages are saved to an internal buffer, and can be retrieved with [`Stderr::buffer`].
    Buffered,
}

impl Stderr {
    /// Creates a new terminal.
    pub fn new(mode: WriteMode) -> Self {
        Stderr {
            mode,
            buffer: String::new(),
            progress: String::new(),
        }
    }

    /// Prints to the standard error, with a newline.
    pub fn eprintln(&mut self, message: &str) {
        match self.mode {
            WriteMode::Immediate => {
                let has_status = !self.progress.is_empty();
                if has_status {
                    // This is the "EL - Erase in Line" sequence. It clears from the cursor
                    // to the end of line.
                    // https://en.wikipedia.org/wiki/ANSI_escape_code#CSI_sequences
                    eprint!("\x1B[K");
                }
                eprintln!("{message}");
                if has_status {
                    eprint!("{}", self.progress);
                }
            }
            WriteMode::Buffered => {
                self.buffer.push_str(message);
                self.buffer.push('\n');
            }
        }
    }

    #[allow(dead_code)]
    /// Sets the progress string (only in [`WriteMode::Immediate`] mode).
    pub fn set_progress(&mut self, progress: &str) {
        match self.mode {
            WriteMode::Immediate => {
                self.progress = progress.to_string();
                eprint!("{}", self.progress);
            }
            WriteMode::Buffered => {}
        }
    }

    #[allow(dead_code)]
    /// Returns the buffered standard error.
    pub fn buffer(&self) -> &str {
        &self.buffer
    }
}

#[cfg(test)]
mod tests {
    use crate::util::term::{Stderr, WriteMode};

    #[test]
    fn term_buffered() {
        let mut term = Stderr::new(WriteMode::Buffered);
        term.eprintln("toto");
        term.set_progress("some progress...\r");
        term.eprintln("tutu");

        assert_eq!(term.buffer(), "toto\ntutu\n")
    }
}
