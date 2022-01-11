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

/**
 * Serde json son can not be easily used for serialization here because of the orphan rule.
 * It seems easier just to reimplement it from scratch (around 50 lines of code)
 */

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JValue {
    Number(String),
    String(String),
    Boolean(bool),
    List(Vec<JValue>),
    Object(Vec<(String, JValue)>),
    Null,
}

impl JValue {
    pub fn format(self) -> String {
        match self {
            JValue::Null => "null".to_string(),
            JValue::Number(n) => n,
            JValue::String(s) => format!("\"{}\"", s.chars().map(format_char).collect::<String>()),
            JValue::Boolean(b) => b.to_string(),
            JValue::List(elem) => {
                let s = elem
                    .iter()
                    .map(|e| e.clone().format())
                    .collect::<Vec<String>>()
                    .join(",");
                format!("[{}]", s)
            }
            JValue::Object(key_values) => {
                let s = key_values
                    .iter()
                    .map(|(k, v)| format!("\"{}\":{}", k, v.clone().format()))
                    .collect::<Vec<String>>()
                    .join(",");
                format!("{{{}}}", s)
            }
        }
    }
}

fn format_char(c: char) -> String {
    if c == '"' {
        "\\\"".to_string()
    } else if c == '\\' {
        "\\\\".to_string()
    } else if c == '\x08' {
        "\\b".to_string()
    } else if c == '\x0c' {
        "\\f".to_string()
    } else if c == '\n' {
        "\\n".to_string()
    } else if c == '\r' {
        "\\r".to_string()
    } else if c == '\t' {
        "\\t".to_string()
    } else if c.is_control() {
        format!("\\u{:04x}", c as u32)
    } else {
        c.to_string()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_format_char() {
        assert_eq!(format_char('a'), "a");
        assert_eq!(format_char('"'), "\\\""); //    \"
        assert_eq!(format_char('\n'), "\\n");
        assert_eq!(format_char('\x07'), "\\u0007")
    }

    #[test]
    pub fn format_scalars() {
        assert_eq!(JValue::Null {}.format(), "null");
        assert_eq!(JValue::Number("1.0".to_string()).format(), "1.0");
        assert_eq!(JValue::String("hello".to_string()).format(), "\"hello\"");
        assert_eq!(JValue::Boolean(true).format(), "true");
    }

    #[test]
    pub fn format_string() {
        assert_eq!(JValue::String("hello".to_string()).format(), "\"hello\"");
        assert_eq!(JValue::String("\"".to_string()).format(), r#""\"""#);
    }

    #[test]
    pub fn format_list() {
        assert_eq!(
            JValue::List(vec![
                JValue::Number("1".to_string()),
                JValue::Number("2".to_string()),
                JValue::Number("3".to_string())
            ])
            .format(),
            "[1,2,3]"
        );
    }

    #[test]
    pub fn format_object() {
        assert_eq!(
            JValue::Object(vec![
                ("name".to_string(), JValue::String("Bob".to_string())),
                ("age".to_string(), JValue::Number("20".to_string())),
            ])
            .format(),
            r#"{"name":"Bob","age":20}"#
        );
    }
}
