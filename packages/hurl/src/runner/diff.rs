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

use hurl_core::text::{Style, StyledString};
use similar::{ChangeTag, DiffOp, DiffTag, TextDiff};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DiffHunk {
    pub content: StyledString,
    pub start: usize,       // 0-based
    pub source_line: usize, // 0-based
}

pub fn diff(old: &str, new: &str) -> Vec<DiffHunk> {
    let text_diff = TextDiff::from_lines(old, new);
    let mut unified_diff = text_diff.unified_diff();
    let unified_diff = unified_diff.context_radius(0);

    let mut hunks = vec![];
    for hunk in unified_diff.iter_hunks() {
        let (start, source_line) = get_hunk_lines(hunk.ops());
        let mut content = StyledString::new();
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
            content.push_with(&line, style);
        }
        let hunk = DiffHunk {
            content,
            source_line,
            start,
        };
        hunks.push(hunk);
    }
    hunks
}

/// get start of the hunk and first change in the input source string
/// Both are 0-based line number
fn get_hunk_lines(ops: &[DiffOp]) -> (usize, usize) {
    let mut start = 0;
    for op in ops {
        match op.tag() {
            DiffTag::Equal => {
                start = op.old_range().start;
            }
            DiffTag::Delete => return (start, op.old_range().start),
            DiffTag::Insert => return (start, op.old_range().start - 1),
            DiffTag::Replace => return (start, op.old_range().start),
        }
    }
    (start, 0)
}

#[cfg(test)]
mod tests {
    use hurl_core::text::{Style, StyledString};

    use super::*;

    fn old_string() -> String {
        r#"{
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
"#
        .to_string()
    }

    fn new_string_change_line1() -> String {
        r#"[
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
"#
        .to_string()
    }

    fn new_string_change_line2() -> String {
        r#"{
  "first_name": "Bob",
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
"#
        .to_string()
    }

    fn new_string_change_line3() -> String {
        r#"{
  "first_name": "John",
  "last_name": "Smiths",
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
"#
        .to_string()
    }

    fn new_string_change_line4() -> String {
        r#"{
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
"#
        .to_string()
    }

    fn new_string_delete_line3() -> String {
        r#"{
  "first_name": "John",
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
"#
        .to_string()
    }

    fn new_string_add_line3() -> String {
        r#"{
  "first_name": "John",
  "middle_name": "Bob",
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
"#
        .to_string()
    }

    fn new_string_change_line4_line24() -> String {
        r#"{
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
    "Bob"
  ],
  "spouse": null
}
"#
        .to_string()
    }

    #[test]
    fn test_diff_change_line1() {
        let mut expected_diff_output = StyledString::new();
        expected_diff_output.push_with(
            r#"-{
"#,
            Style::new().red(),
        );
        expected_diff_output.push_with(
            r#"+[
"#,
            Style::new().green(),
        );

        let hunks = diff(&old_string(), &new_string_change_line1());
        let first_hunk = hunks.first().unwrap().clone();
        assert_eq!(first_hunk.content, expected_diff_output);
        assert_eq!(first_hunk.start, 0);
        assert_eq!(first_hunk.source_line, 0);
    }

    #[test]
    fn test_diff_change_line2() {
        let mut expected_diff_output = StyledString::new();
        expected_diff_output.push_with(
            r#"-  "first_name": "John",
"#,
            Style::new().red(),
        );
        expected_diff_output.push_with(
            r#"+  "first_name": "Bob",
"#,
            Style::new().green(),
        );

        let hunks = diff(&old_string(), &new_string_change_line2());
        let first_hunk = hunks.first().unwrap().clone();
        assert_eq!(first_hunk.content, expected_diff_output);
        assert_eq!(first_hunk.start, 1);
        assert_eq!(first_hunk.source_line, 1);
    }

    #[test]
    fn test_diff_change_line3() {
        let mut expected_diff_output = StyledString::new();
        expected_diff_output.push_with(
            r#"-  "last_name": "Smith",
"#,
            Style::new().red(),
        );
        expected_diff_output.push_with(
            r#"+  "last_name": "Smiths",
"#,
            Style::new().green(),
        );

        let hunks = diff(&old_string(), &new_string_change_line3());
        let first_hunk = hunks.first().unwrap().clone();
        assert_eq!(first_hunk.content, expected_diff_output);
        assert_eq!(first_hunk.start, 2);
        assert_eq!(first_hunk.source_line, 2);
    }

    #[test]
    fn test_diff_change_line4() {
        let mut expected_diff_output = StyledString::new();
        expected_diff_output.push_with(
            r#"-  "age": 27,
"#,
            Style::new().red(),
        );
        expected_diff_output.push_with(
            r#"+  "age": 28,
"#,
            Style::new().green(),
        );
        let hunks = diff(&old_string(), &new_string_change_line4());

        let first_hunk = hunks.first().unwrap().clone();
        assert_eq!(first_hunk.content, expected_diff_output);
        assert_eq!(first_hunk.start, 4);
        assert_eq!(first_hunk.source_line, 4);
    }

    #[test]
    fn test_diff_delete_line3() {
        let mut expected_diff_output = StyledString::new();
        expected_diff_output.push_with(
            r#"-  "last_name": "Smith",
"#,
            Style::new().red(),
        );

        let hunks = diff(&old_string(), &new_string_delete_line3());
        let first_hunk = hunks.first().unwrap().clone();
        assert_eq!(first_hunk.content, expected_diff_output);
        assert_eq!(first_hunk.start, 2);
        assert_eq!(first_hunk.source_line, 2);
    }

    #[test]
    fn test_diff_add_line3() {
        let mut expected_diff_output = StyledString::new();
        expected_diff_output.push_with(
            r#"+  "middle_name": "Bob",
"#,
            Style::new().green(),
        );

        let hunks = diff(&old_string(), &new_string_add_line3());
        let first_hunk = hunks.first().unwrap().clone();
        assert_eq!(first_hunk.content, expected_diff_output);
        assert_eq!(first_hunk.start, 2);
        assert_eq!(first_hunk.source_line, 1);
    }

    #[test]
    fn test_diff_change_line4_line24() {
        let hunks = diff(&old_string(), &new_string_change_line4_line24());

        let first_hunk = hunks.first().unwrap().clone();
        let mut expected_diff_output = StyledString::new();
        expected_diff_output.push_with(
            r#"-  "age": 27,
"#,
            Style::new().red(),
        );
        expected_diff_output.push_with(
            r#"+  "age": 28,
"#,
            Style::new().green(),
        );
        assert_eq!(first_hunk.content, expected_diff_output);
        assert_eq!(first_hunk.start, 4);
        assert_eq!(first_hunk.source_line, 4);

        let second_hunk = hunks.get(1).unwrap().clone();
        let mut expected_diff_output = StyledString::new();
        expected_diff_output.push_with(
            r#"-    "Trevor"
"#,
            Style::new().red(),
        );
        expected_diff_output.push_with(
            r#"+    "Bob"
"#,
            Style::new().green(),
        );
        assert_eq!(second_hunk.content, expected_diff_output);
        assert_eq!(second_hunk.start, 24);
        assert_eq!(second_hunk.source_line, 24);
    }

    #[test]
    fn test_diff_add_new_line() {
        let hunks = diff("<p>Hello</p>\n", "<p>Hello</p>\n\n");
        let first_hunk = hunks.first().unwrap().clone();

        let mut expected_diff_output = StyledString::new();
        expected_diff_output.push_with("+\n", Style::new().green());

        assert_eq!(first_hunk.content, expected_diff_output);
        assert_eq!(first_hunk.start, 1);
        assert_eq!(first_hunk.source_line, 0);
    }
}
