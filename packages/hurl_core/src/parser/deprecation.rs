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
use crate::ast::SourceInfo;
use crate::error;
use crate::error::DisplaySourceError;
use crate::reader::Pos;
use crate::text::{Style, StyledString};

/// Represents a parser error.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseDeprecation {
    pub pos: Pos,
    pub kind: ParseDeprecationKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseDeprecationKind {
    FormatFilter,
}

impl ParseDeprecation {
    /// Creates a new error for the position `pos`, of type `inner`.
    pub fn new(pos: Pos, kind: ParseDeprecationKind) -> ParseDeprecation {
        ParseDeprecation { pos, kind }
    }
}

impl DisplaySourceError for ParseDeprecation {
    fn source_info(&self) -> SourceInfo {
        SourceInfo {
            start: self.pos,
            end: self.pos,
        }
    }

    fn description(&self) -> String {
        match self.kind {
            ParseDeprecationKind::FormatFilter => "Parsing 'format' filter".to_string(),
        }
    }

    fn fixme(&self, content: &[&str]) -> StyledString {
        let message = match &self.kind {
            ParseDeprecationKind::FormatFilter => {
                "deprecated in favor of the 'dateFormat' filter".to_string()
            }
        };

        let message = error::add_carets(&message, self.source_info(), content);
        let mut s = StyledString::new();
        s.push_with(&message, Style::new().red().bold());
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::OutputFormat;

    #[test]
    fn test_parsing_deprecation() {
        let content = r#"GET https://example.com
HTTP 200
[Captures]
myDate: jsonpath "$.dateTime" format "%a, %d %b %Y %H:%M:%S"
"#;
        let filename = "test.hurl";
        let error = ParseDeprecation {
            pos: Pos::new(4, 31),
            kind: ParseDeprecationKind::FormatFilter,
        };
        assert_eq!(
            error.render(filename, content, None, OutputFormat::Terminal(false)),
            r#"Parsing 'format' filter
  --> test.hurl:4:31
   |
 4 | myDate: jsonpath "$.dateTime" format "%a, %d %b %Y %H:%M:%S"
   |                               ^ deprecated in favor of the 'dateFormat' filter
   |"#
        );
    }
}
