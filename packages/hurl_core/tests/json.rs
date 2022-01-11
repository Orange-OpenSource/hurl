/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2022 Orange
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
use hurl_core::ast::JsonValue;
use std::fs;
use std::fs::File;
use std::io::Read;

#[test]
fn debug() {
    let content = r#""{{name}}""#;
    let value = parse_json(content.to_string()).unwrap();
    eprintln!("{}", value.encoded());
}

#[test]
fn test_echo() {
    for file in json_files() {
        eprintln!("{}", file);
        let content = read_content(file);
        let value = parse_json(content.to_string()).unwrap();
        assert_eq!(value.encoded(), content);
    }
}

fn parse_json(content: String) -> Result<JsonValue, hurl_core::parser::Error> {
    let mut reader = hurl_core::parser::Reader::init(content.as_str());
    hurl_core::parser::parse_json(&mut reader)
}

fn read_content(filename: String) -> String {
    let mut file = File::open(filename.as_str()).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn json_files() -> Vec<String> {
    let mut paths = vec![];
    for dir_entry in fs::read_dir("tests/json").expect("tests/json") {
        let path = dir_entry.expect("dir_entry").path();
        if path.to_str().unwrap().ends_with("json") {
            paths.push(path.to_str().unwrap().to_string());
        }
    }
    paths.sort();
    paths
}
