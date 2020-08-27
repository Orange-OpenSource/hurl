/*
 * hurl (https://hurl.dev)
 * Copyright (C) 2020 Orange
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

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use crate::core::common::Value;

use super::core::{Error, RunnerError};
use super::super::core::ast::*;

impl Body {
    pub fn eval(self, variables: &HashMap<String, Value>, context_dir: String) -> Result<Vec<u8>, Error> {
        self.value.eval(variables, context_dir)
    }
}

impl Bytes {
    pub fn eval(self, variables: &HashMap<String, Value>, context_dir: String) -> Result<Vec<u8>, Error> {
        match self {
            Bytes::RawString { value, .. } => {
                let value = value.eval(variables)?;
                Ok(value.into_bytes())
            }
            Bytes::Base64 { value, .. } => Ok(value),
            Bytes::Xml { value, .. } => Ok(value.into_bytes()),
            Bytes::Json { value, .. } => {
                let value = value.eval(variables)?;
                Ok(value.into_bytes())
            }
            Bytes::File { filename, .. } => {
                let path = Path::new(filename.value.as_str());
                let absolute_filename = if path.is_absolute() {
                    filename.clone().value
                } else {
                    Path::new(context_dir.as_str()).join(filename.value).to_str().unwrap().to_string()
                };
                match File::open(absolute_filename.clone()) {
                    Ok(f) => {
                        let mut bytes = vec![];
                        for byte in f.bytes() {
                            bytes.push(byte.unwrap());
                        }
                        Ok(bytes)
                    }
                    Err(_) => Err(Error {
                        source_info: filename.source_info,
                        inner: RunnerError::FileReadAccess { value: absolute_filename },
                        assert: false,
                    })
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::core::common::SourceInfo;

    use super::*;

    #[cfg(test)]
    pub fn create_test_file() {
        let path = Path::new("/tmp/data.bin");
        let display = path.display();
        if File::open(path).is_err() {
            match File::create(&path) {
                Err(why) => panic!("couldn't create {}: {:?}", display, why),
                Ok(mut file) => match file.write_all(b"Hello World!") {
                    Err(why) => panic!("couldn't write to {}: {:?}", display, why),
                    Ok(_) => println!("successfully wrote to {}", display),
                }
            }
        }
    }

    #[test]
    pub fn test_body_file() {
        create_test_file();

        // file, data.bin;
        let whitespace = Whitespace {
            value: String::from(" "),
            source_info: SourceInfo::init(0, 0, 0, 0),
        };

        let bytes = Bytes::File {
            space0: whitespace.clone(),
            filename: Filename { value: String::from("/tmp/data.bin"), source_info: SourceInfo::init(1, 7, 1, 15) },
            space1: whitespace.clone(),
        };

        let variables = HashMap::new();
        assert_eq!(bytes.eval(&variables, "current_dir".to_string()).unwrap(), b"Hello World!");
    }

    #[test]
    pub fn test_body_file_error() {
        // file, data.bin;
        let whitespace = Whitespace {
            value: String::from(" "),
            source_info: SourceInfo::init(0, 0, 0, 0),
        };

        let bytes = Bytes::File {
            space0: whitespace.clone(),
            filename: Filename { value: String::from("data.bin"), source_info: SourceInfo::init(1, 7, 1, 15) },
            space1: whitespace.clone(),
        };

        let variables = HashMap::new();

        let error = bytes.eval(&variables, "current_dir".to_string()).err().unwrap();
        assert_eq!(error.inner, RunnerError::FileReadAccess { value: String::from("current_dir/data.bin") });
        assert_eq!(error.source_info, SourceInfo::init(1, 7, 1, 15));
    }
}
