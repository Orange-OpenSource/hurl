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
use std::fs::File;
use std::io::{ErrorKind, Read};
use std::path::{Path, PathBuf};
use std::{fmt, fs, io};

/// Represents the input of read operation: can be either a file or standard input.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Input {
    /// Read from file.
    File(PathBuf),
    /// Read from standard input.
    Stdin,
}

impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = match self {
            Input::File(file) => file.to_string_lossy().to_string(),
            Input::Stdin => "-".to_string(),
        };
        write!(f, "{output}")
    }
}

impl From<&Path> for Input {
    fn from(value: &Path) -> Self {
        Input::File(value.to_path_buf())
    }
}

impl From<PathBuf> for Input {
    fn from(value: PathBuf) -> Self {
        Input::File(value)
    }
}

impl Input {
    /// Creates a new input from a string filename.
    pub fn new(filename: &str) -> Self {
        if filename == "-" {
            Input::Stdin
        } else {
            Input::File(PathBuf::from(filename))
        }
    }

    /// Reads the content of this input to a string, removing any BOM.
    pub fn read_to_string(&self) -> Result<String, io::Error> {
        match self {
            Input::File(path) => {
                let mut f = File::open(path)?;
                let metadata = fs::metadata(path).unwrap();
                let mut buffer = vec![0; metadata.len() as usize];
                f.read_exact(&mut buffer)?;
                string_from_utf8(buffer)
            }
            Input::Stdin => {
                let mut contents = String::new();
                io::stdin().read_to_string(&mut contents)?;
                Ok(contents)
            }
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
