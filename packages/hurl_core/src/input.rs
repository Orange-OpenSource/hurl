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
use std::fs::File;
use std::io::{ErrorKind, Read};
use std::path::{Path, PathBuf};
use std::{fmt, fs, io};

/// Represents the input of read operation: can be either a file or standard input.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Input {
    /// Kind of input: either sourced from a file source, or from standard input.
    kind: InputKind,
}

impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.kind.fmt(f)
    }
}

impl Input {
    /// Creates an input from a path source.
    pub fn new(path: &str) -> Self {
        let kind = InputKind::File(PathBuf::from(path));
        Input { kind }
    }

    /// Creates an input from standard input.
    /// The content of the standard input is read once and then cached. It can be re-read multiple
    /// times.
    pub fn from_stdin() -> Result<Self, io::Error> {
        let mut contents = String::new();
        io::stdin().read_to_string(&mut contents)?;
        let kind = InputKind::Stdin(contents);
        Ok(Input { kind })
    }

    pub fn kind(&self) -> &InputKind {
        &self.kind
    }

    /// Reads the content of this input to a string, removing any BOM.
    pub fn read_to_string(&self) -> Result<String, io::Error> {
        self.kind.read_to_string()
    }
}

/// Represents the kind of input of read operation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InputKind {
    /// Read from file.
    File(PathBuf),
    /// Read from standard input. Input is read once and the stdin string is cached and can be read
    /// multiple times.
    Stdin(String),
}

impl fmt::Display for InputKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = match self {
            InputKind::File(file) => file.to_string_lossy().to_string(),
            InputKind::Stdin(_) => "-".to_string(),
        };
        write!(f, "{output}")
    }
}

impl From<&Path> for Input {
    fn from(value: &Path) -> Self {
        let kind = InputKind::File(value.to_path_buf());
        Input { kind }
    }
}

impl From<PathBuf> for Input {
    fn from(value: PathBuf) -> Self {
        let kind = InputKind::File(value);
        Input { kind }
    }
}

impl InputKind {
    /// Reads the content of this input to a string, removing any BOM.
    fn read_to_string(&self) -> Result<String, io::Error> {
        match self {
            InputKind::File(path) => {
                let mut f = File::open(path)?;
                let metadata = fs::metadata(path).unwrap();
                let mut buffer = vec![0; metadata.len() as usize];
                f.read_exact(&mut buffer)?;
                string_from_utf8(buffer)
            }
            InputKind::Stdin(cached) => Ok(cached.clone()),
        }
    }
}

fn string_from_utf8(buffer: Vec<u8>) -> Result<String, io::Error> {
    let mut buffer = buffer;
    strip_bom(&mut buffer);
    String::from_utf8(buffer).map_err(|e| io::Error::new(ErrorKind::InvalidData, e))
}

/// Remove BOM from the input bytes
fn strip_bom(bytes: &mut Vec<u8>) {
    if bytes.starts_with(&[0xefu8, 0xbb, 0xbf]) {
        bytes.drain(0..3);
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_strip_bom() {
        let mut bytes = vec![];
        strip_bom(&mut bytes);
        assert!(bytes.is_empty());

        let mut bytes = vec![0xef, 0xbb, 0xbf, 0x68, 0x65, 0x6c, 0x6c, 0x6f];
        strip_bom(&mut bytes);
        assert_eq!(bytes, vec![0x68, 0x65, 0x6c, 0x6c, 0x6f]);

        let mut bytes = vec![0x68, 0x65, 0x6c, 0x6c, 0x6f];
        strip_bom(&mut bytes);
        assert_eq!(bytes, vec![0x68, 0x65, 0x6c, 0x6c, 0x6f]);
    }

    #[test]
    fn test_string_from_utf8_bom() {
        let mut bytes = vec![];
        strip_bom(&mut bytes);
        assert_eq!(string_from_utf8(vec![]).unwrap(), "");
        assert_eq!(
            string_from_utf8(vec![0xef, 0xbb, 0xbf, 0x68, 0x65, 0x6c, 0x6c, 0x6f]).unwrap(),
            "hello"
        );
        assert_eq!(
            string_from_utf8(vec![0x68, 0x65, 0x6c, 0x6c, 0x6f]).unwrap(),
            "hello"
        );
        let err = string_from_utf8(vec![0xef]).err().unwrap();
        assert_eq!(
            err.to_string(),
            "incomplete utf-8 byte sequence from index 0"
        );
    }
}
