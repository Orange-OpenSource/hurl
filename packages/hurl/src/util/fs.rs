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
use std::{fs, io};

/// Read the content of `filename` to a string.
///
/// This method is similar to the standard [`std::io::Stdin::read_to_string()`] but remove any
/// existing BOM. It supports also input stream when filename = '-'.
pub fn read_to_string(filename: &str) -> Result<String, io::Error> {
    if filename == "-" {
        let mut contents = String::new();
        io::stdin().read_to_string(&mut contents)?;
        return Ok(contents);
    }
    let mut f = File::open(filename)?;
    let metadata = fs::metadata(filename).unwrap();
    let mut buffer = vec![0; metadata.len() as usize];
    f.read_exact(&mut buffer)?;
    string_from_utf8(buffer)
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
