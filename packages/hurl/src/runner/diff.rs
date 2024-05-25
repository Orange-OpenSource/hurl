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
use colored::*;
use similar::{ChangeTag, TextDiff};

#[allow(dead_code)]
pub fn diff(expected: &str, actual: &str, color: bool) -> String {
    let text_diff = TextDiff::from_lines(expected, actual);
    let unified_diff = text_diff.unified_diff();

    let mut s = String::new();
    for hunk in unified_diff.iter_hunks() {
        for change in hunk.iter_changes() {
            let sign = match change.tag() {
                ChangeTag::Delete => "-",
                ChangeTag::Insert => "+",
                ChangeTag::Equal => " ",
            };

            let mut line = format!("{}{}", sign, change);
            if color {
                line = match change.tag() {
                    ChangeTag::Delete => line.red().to_string(),
                    ChangeTag::Insert => line.green().to_string(),
                    ChangeTag::Equal => line.clone(),
                };
            }

            s.push_str(line.as_str());
        }
    }

    s.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff_json_strings() {
        let old = r#"{
  "first_name": "John",
  "last_name": "Smith",
  "is_alive": true,
  "age": 27,
  "address": {
    "street_address": "21 2nd Street",
    "city": "New York",
    "state": "NY",
    "postal_code": "10021-3100"
  },
  "phone_numbers": [
    {
      "type": "home",
      "number": "212 555-1234"
    },
    {
      "type": "office",
      "number": "646 555-4567"
    }
  ],
  "children": [
    "Catherine",
    "Thomas",
    "Trevor"
  ],
  "spouse": null
}
"#;

        let new = r#"{
  "first_name": "John",
  "last_name": "Smith",
  "is_alive": true,
  "age": 28,
  "address": {
    "street_address": "21 2nd Street",
    "city": "New York",
    "state": "NY",
    "postal_code": "10021-3100"
  },
  "phone_numbers": [
    {
      "type": "home",
      "number": "212 555-1234"
    },
    {
      "type": "office",
      "number": "646 555-4567"
    }
  ],
  "children": [
    "Catherine",
    "Thomas",
    "Trevor"
  ],
  "spouse": null
}
"#;

        let diff_output = r#"   "first_name": "John",
   "last_name": "Smith",
   "is_alive": true,
-  "age": 27,
+  "age": 28,
   "address": {
     "street_address": "21 2nd Street",
     "city": "New York",
"#;
        assert_eq!(diff(old, new, false), diff_output);

        control::set_override(true);
        let diff_colored_output = "   \"first_name\": \"John\",\n   \"last_name\": \"Smith\",\n   \"is_alive\": true,\n\u{1b}[31m-  \"age\": 27,\n\u{1b}[0m\u{1b}[32m+  \"age\": 28,\n\u{1b}[0m   \"address\": {\n     \"street_address\": \"21 2nd Street\",\n     \"city\": \"New York\",\n";
        assert_eq!(diff(old, new, true), diff_colored_output);
    }
}
