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
extern crate reqwest;

use std::fmt;

use serde::{Deserialize, Serialize};

pub enum Encoding {
    Utf8,
    Latin1,
}

impl fmt::Display for Encoding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Encoding::Utf8 => "utf8",
            Encoding::Latin1 => "iso8859-1"
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Url {
    pub scheme: String,
    pub host: String,
    pub port: Option<u16>,
    pub path: String,
    pub query_string: String,
}


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Header {
    pub name: String,
    pub value: String,
}

pub fn get_header_value(headers: Vec<Header>, name: &str) -> Option<String> {
    for header in headers {
        if header.name.as_str() == name {
            return Some(header.value);
        }
    }
    None
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Param {
    pub name: String,
    pub value: String,
}

pub fn encode_form_params(params: Vec<Param>) -> Vec<u8> {
    params
        .iter()
        //.map(|p| format!("{}={}", p.name, utf8_percent_encode(p.value.as_str(), FRAGMENT)))
        .map(|p| format!("{}={}", p.name, url_encode(p.value.clone())))
        .collect::<Vec<_>>()
        .join("&")
        .into_bytes()
}


fn url_encode(s: String) -> String {
    const MAX_CHAR_VAL: u32 = std::char::MAX as u32;
    let mut buff = [0; 4];
    s.chars()
        .map(|ch| {
            match ch as u32 {
                0..=47 | 58..=64 | 91..=96 | 123..=MAX_CHAR_VAL => {
                    ch.encode_utf8(&mut buff);
                    buff[0..ch.len_utf8()].iter().map(|&byte| format!("%{:x}", byte)).collect::<String>()
                }
                _ => ch.to_string(),
            }
        })
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_form_params() {
        assert_eq!(
            encode_form_params(vec![
                Param {
                    name: String::from("param1"),
                    value: String::from("value1"),
                },
                Param {
                    name: String::from("param2"),
                    value: String::from(""),
                }
            ]),
            vec![
                112, 97, 114, 97, 109, 49, 61, 118, 97, 108, 117, 101, 49, 38, 112, 97, 114, 97, 109,
                50, 61
            ]
        );
        assert_eq!(
            std::str::from_utf8(&encode_form_params(vec![
                Param { name: String::from("param1"), value: String::from("value1") },
                Param { name: String::from("param2"), value: String::from("") },
                Param { name: String::from("param3"), value: String::from("a=b") },
                Param { name: String::from("param4"), value: String::from("a%3db") },
            ])).unwrap(),
            "param1=value1&param2=&param3=a%3db&param4=a%253db"
        );
    }
}
