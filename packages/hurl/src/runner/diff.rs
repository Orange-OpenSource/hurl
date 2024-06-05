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

use hurl_core::text::{Style, StyledString};
use similar::{ChangeTag, TextDiff};

#[allow(dead_code)]
pub fn diff(expected: &str, actual: &str) -> StyledString {
    let text_diff = TextDiff::from_lines(expected, actual);
    let unified_diff = text_diff.unified_diff();

    let mut s = StyledString::new();
    for hunk in unified_diff.iter_hunks() {
        for change in hunk.iter_changes() {
            let sign = match change.tag() {
                ChangeTag::Delete => "-",
                ChangeTag::Insert => "+",
                ChangeTag::Equal => " ",
            };
            let line = format!("{}{}", sign, change);
            let style = match change.tag() {
                ChangeTag::Delete => Style::new().red(),
                ChangeTag::Insert => Style::new().green(),
                ChangeTag::Equal => Style::new(),
            };
            s.push_with(&line, style);
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use hurl_core::text::{Format, Style, StyledString};

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

        let mut diff_output = StyledString::new();
        diff_output.push(
            r#"   "first_name": "John",
   "last_name": "Smith",
   "is_alive": true,
"#,
        );
        diff_output.push_with(
            r#"-  "age": 27,
"#,
            Style::new().red(),
        );
        diff_output.push_with(
            r#"+  "age": 28,
"#,
            Style::new().green(),
        );
        diff_output.push(
            r#"   "address": {
     "street_address": "21 2nd Street",
     "city": "New York",
"#,
        );

        assert_eq!(diff(old, new), diff_output);

        let diff_output_plain = r#"   "first_name": "John",
   "last_name": "Smith",
   "is_alive": true,
-  "age": 27,
+  "age": 28,
   "address": {
     "street_address": "21 2nd Street",
     "city": "New York",
"#;
        assert_eq!(diff(old, new).to_string(Format::Plain), diff_output_plain);

        colored::control::set_override(true);
        let diff_output_colored = "   \"first_name\": \"John\",\n   \"last_name\": \"Smith\",\n   \"is_alive\": true,\n\u{1b}[31m-  \"age\": 27,\n\u{1b}[0m\u{1b}[32m+  \"age\": 28,\n\u{1b}[0m   \"address\": {\n     \"street_address\": \"21 2nd Street\",\n     \"city\": \"New York\",\n";
        assert_eq!(diff(old, new).to_string(Format::Ansi), diff_output_colored);
    }
}
