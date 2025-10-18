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
use hurl_core::ast::visit::Visitor;
use hurl_core::ast::{
    Comment, CookiePath, FilterValue, HurlFile, JsonValue, Method, MultilineString, Number,
    Placeholder, PredicateFuncValue, QueryValue, Regex, StatusValue, Template, VersionValue,
    Whitespace, U64,
};
use hurl_core::text::{Format, Style, StyledString};
use hurl_core::types::{DurationUnit, SourceString, ToSource};

/// Format a Hurl `file` to text, using ANSI escape code if `color` is true.
pub fn format(file: &HurlFile, color: bool) -> String {
    let format = if color { Format::Ansi } else { Format::Plain };
    let mut fmt = TextFormatter::new();
    fmt.format(file, format)
}

/// A formatter for text and terminal export.
struct TextFormatter {
    buffer: StyledString,
}

impl TextFormatter {
    /// Creates a new text formatter supporting ANSI escape code coloring.
    fn new() -> Self {
        TextFormatter {
            buffer: StyledString::new(),
        }
    }

    fn format(&mut self, file: &HurlFile, format: Format) -> String {
        self.buffer.clear();
        self.visit_hurl_file(file);
        self.buffer.to_string(format)
    }
}

impl Visitor for TextFormatter {
    fn visit_base64_value(&mut self, _value: &[u8], source: &SourceString) {
        self.buffer.push_with(source.as_str(), Style::new().green());
    }

    fn visit_bool(&mut self, value: bool) {
        let value = value.to_string();
        self.buffer.push_with(value.as_str(), Style::new().cyan());
    }

    fn visit_cookie_path(&mut self, path: &CookiePath) {
        let value = path.to_source();
        self.buffer.push_with(value.as_str(), Style::new().green());
    }

    fn visit_comment(&mut self, comment: &Comment) {
        let value = comment.to_source();
        self.buffer
            .push_with(value.as_str(), Style::new().bright_black());
    }

    fn visit_duration_unit(&mut self, unit: DurationUnit) {
        let value = unit.to_string();
        self.buffer.push_with(&value, Style::new().cyan());
    }

    fn visit_filename(&mut self, filename: &Template) {
        let value = filename.to_source();
        self.buffer.push_with(value.as_str(), Style::new().green());
    }

    fn visit_filter_kind(&mut self, kind: &FilterValue) {
        let value = kind.identifier();
        self.buffer.push_with(value, Style::new().yellow());
    }

    fn visit_hex_value(&mut self, _value: &[u8], source: &SourceString) {
        self.buffer.push_with(source.as_str(), Style::new().green());
    }

    fn visit_i64(&mut self, n: i64) {
        let value = n.to_string();
        self.buffer.push_with(&value, Style::new().cyan());
    }

    fn visit_json_body(&mut self, json: &JsonValue) {
        let value = json.to_source();
        self.buffer.push_with(value.as_str(), Style::new().green());
    }

    fn visit_literal(&mut self, lit: &'static str) {
        self.buffer.push(lit);
    }

    fn visit_method(&mut self, method: &Method) {
        let value = method.to_source();
        self.buffer.push_with(value.as_str(), Style::new().yellow());
    }

    fn visit_multiline_string(&mut self, string: &MultilineString) {
        let value = string.to_source();
        self.buffer.push_with(value.as_str(), Style::new().green());
    }

    fn visit_not(&mut self, identifier: &'static str) {
        self.buffer.push_with(identifier, Style::new().yellow());
    }

    fn visit_null(&mut self, identifier: &'static str) {
        self.buffer.push_with(identifier, Style::new().cyan());
    }

    fn visit_number(&mut self, number: &Number) {
        let value = number.to_source();
        self.buffer.push_with(value.as_str(), Style::new().cyan());
    }

    fn visit_placeholder(&mut self, placeholder: &Placeholder) {
        let value = placeholder.to_source();
        self.buffer.push_with(value.as_str(), Style::new().green());
    }

    fn visit_predicate_kind(&mut self, kind: &PredicateFuncValue) {
        let value = kind.identifier();
        self.buffer.push_with(value, Style::new().yellow());
    }

    fn visit_query_kind(&mut self, kind: &QueryValue) {
        let value = kind.identifier();
        self.buffer.push_with(value, Style::new().cyan());
    }

    fn visit_regex(&mut self, regex: &Regex) {
        let value = regex.to_source();
        self.buffer.push_with(value.as_str(), Style::new().green());
    }

    fn visit_status(&mut self, value: &StatusValue) {
        let value = value.to_string();
        self.buffer.push(&value);
    }

    fn visit_string(&mut self, value: &str) {
        self.buffer.push_with(value, Style::new().green());
    }

    fn visit_section_header(&mut self, name: &str) {
        self.buffer.push_with(name, Style::new().magenta());
    }

    fn visit_template(&mut self, template: &Template) {
        let value = template.to_source();
        self.buffer.push_with(value.as_str(), Style::new().green());
    }

    fn visit_url(&mut self, url: &Template) {
        let value = url.to_source();
        self.buffer.push_with(value.as_str(), Style::new().green());
    }

    fn visit_u64(&mut self, n: &U64) {
        let value = n.to_source();
        self.buffer.push_with(value.as_str(), Style::new().cyan());
    }

    fn visit_usize(&mut self, n: usize) {
        let value = n.to_string();
        self.buffer.push_with(value.as_str(), Style::new().cyan());
    }

    fn visit_variable_name(&mut self, name: &str) {
        self.buffer.push(name);
    }

    fn visit_version(&mut self, value: &VersionValue) {
        let value = value.to_string();
        self.buffer.push(&value);
    }

    fn visit_xml_body(&mut self, xml: &str) {
        self.buffer.push_with(xml, Style::new().green());
    }

    fn visit_whitespace(&mut self, ws: &Whitespace) {
        self.buffer.push(ws.as_str());
    }
}

#[cfg(test)]
mod tests {
    use crate::format::text::TextFormatter;
    use hurl_core::parser::parse_hurl_file;
    use hurl_core::text::Format;

    #[test]
    fn format_hurl_file() {
        // For the crate colored to output ANSI escape code in test environment.
        hurl_core::text::init_crate_colored();

        let src = r#"
GET https://foo.com
header1: value1
header2: value2
[Form]
foo: bar
baz: 123
HTTP 200
[Asserts]
jsonpath "$.name" == "toto"
"#;
        let file = parse_hurl_file(src).unwrap();
        let mut fmt = TextFormatter::new();
        let dst = fmt.format(&file, Format::Plain);
        assert_eq!(src, dst);

        let dst = fmt.format(&file, Format::Ansi);
        assert_eq!(
            dst,
            r#"
[33mGET[0m [32mhttps://foo.com[0m
[32mheader1[0m: [32mvalue1[0m
[32mheader2[0m: [32mvalue2[0m
[35m[Form][0m
[32mfoo[0m: [32mbar[0m
[32mbaz[0m: [32m123[0m
HTTP 200
[35m[Asserts][0m
[36mjsonpath[0m [32m"$.name"[0m [33m==[0m [32m"toto"[0m
"#
        );
    }
}
