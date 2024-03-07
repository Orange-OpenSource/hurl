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

#[derive(Clone)]
pub struct Term {
    mode: WriteMode,
    stderr: String,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum WriteMode {
    Immediate,
    Buffered,
}

/// Represents a terminal and acts as an indirection for standard output / error.
/// Depending on `mode`, messages are immediately printed to standard output /error, or buffered
/// in strings.
impl Term {
    /// Creates a new terminal.
    pub fn new(mode: WriteMode) -> Self {
        Term {
            mode,
            stderr: String::new(),
        }
    }

    /// Prints to the standard error, with a newline.
    pub fn eprintln(&mut self, message: &str) {
        match self.mode {
            WriteMode::Immediate => eprintln!("{message}"),
            WriteMode::Buffered => {
                self.stderr.push_str(message);
                self.stderr.push('\n');
            }
        }
    }

    #[allow(dead_code)]
    /// Returns the buffered standard error.
    pub fn stderr(&self) -> &str {
        &self.stderr
    }
}

#[cfg(test)]
mod tests {
    use crate::util::term::{Term, WriteMode};

    #[test]
    fn term_buffered() {
        let mut term = Term::new(WriteMode::Buffered);
        term.eprintln("toto");
        term.eprintln("tutu");

        assert_eq!(term.stderr(), "toto\ntutu\n")
    }
}
