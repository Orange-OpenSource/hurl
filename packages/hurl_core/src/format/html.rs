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
use crate::ast::visit::Visitor;
use crate::ast::{
    visit, Comment, Entry, FilterValue, JsonValue, Method, Placeholder, Regex, Request, Response,
    Template, Whitespace, U64,
};
use crate::ast::{
    CookiePath, HurlFile, MultilineString, Number, PredicateFuncValue, QueryValue, StatusValue,
    VersionValue,
};
use crate::types::{DurationUnit, SourceString, ToSource};

/// Returns an HTML string of the Hurl file `hurl_file`.
///
/// If `standalone` is true, a complete HTML body with inline styling is returned.
/// Otherwise, a `<pre>` HTML tag is returned, without styling.
pub fn format(file: &HurlFile, standalone: bool) -> String {
    let mut fmt = HtmlFormatter::new();
    let body = fmt.format(file);
    if standalone {
        let css = include_str!("hurl.css");
        format!(
            r#"<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8">
        <title>Hurl File</title>
        <style>
{css}
        </style>
    </head>
    <body>
{body}
    </body>
</html>
"#
        )
    } else {
        body.to_string()
    }
}

pub fn hurl_css() -> String {
    include_str!("hurl.css").to_string()
}

/// A HTML formatter for Hurl content.
struct HtmlFormatter {
    buffer: String,
}

const HURL_BASE64_VALUE_CLASS: &str = "base64";
const HURL_BOOLEAN_CLASS: &str = "boolean";
const HURL_COMMENT_CLASS: &str = "comment";
const HURL_DURATION_UNIT: &str = "unit";
const HURL_ENTRY_CLASS: &str = "entry";
const HURL_HEX_CLASS: &str = "hex";
const HURL_FILENAME_CLASS: &str = "filename";
const HURL_FILTER_KIND_CLASS: &str = "filter-type";
const HURL_JSON_CLASS: &str = "json";
const HURL_LANG_CLASS: &str = "language-hurl";
const HURL_METHOD_CLASS: &str = "method";
const HURL_MULTILINESTRING_CLASS: &str = "multiline";
const HURL_NULL_CLASS: &str = "null";
const HURL_NUMBER_CLASS: &str = "number";
const HURL_NOT_CLASS: &str = "not";
const HURL_PLACEHOLDER_CLASS: &str = "expr";
const HURL_PREDICATE_TYPE_CLASS: &str = "predicate-type";
const HURL_QUERY_TYPE_CLASS: &str = "query-type";
const HURL_REGEX_CLASS: &str = "regex";
const HURL_REQUEST_CLASS: &str = "request";
const HURL_RESPONSE_CLASS: &str = "response";
const HURL_SECTION_HEADER_CLASS: &str = "section-header";
const HURL_STRING_CLASS: &str = "string";
const HURL_URL_CLASS: &str = "url";
const HURL_VERSION_CLASS: &str = "version";
const HURL_XML_CLASS: &str = "xml";

impl HtmlFormatter {
    /// Creates a new HTML formatter.
    fn new() -> Self {
        HtmlFormatter {
            buffer: String::new(),
        }
    }

    fn format(&mut self, file: &HurlFile) -> &str {
        self.buffer.clear();
        self.visit_hurl_file(file);
        &self.buffer
    }

    fn pre_open(&mut self, class: &'static str) {
        self.buffer.push_str("<pre><code class=\"");
        self.buffer.push_str(class);
        self.buffer.push_str("\">");
    }

    fn pre_close(&mut self) {
        self.buffer.push_str("</code></pre>");
    }

    fn span_open(&mut self, class: &'static str) {
        self.buffer.push_str("<span class=\"");
        self.buffer.push_str(class);
        self.buffer.push_str("\">");
    }

    fn span_close(&mut self) {
        self.buffer.push_str("</span>");
    }

    fn push_source(&mut self, source: &SourceString) {
        // SourceString must be escaped before wrote
        self.push_untrusted(source.as_str());
    }

    fn push_untrusted(&mut self, str: &str) {
        let escaped = str
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;");
        self.buffer.push_str(&escaped);
    }

    fn push_trusted(&mut self, str: &str) {
        self.buffer.push_str(str);
    }
}

impl Visitor for HtmlFormatter {
    fn visit_base64_value(&mut self, _value: &[u8], source: &SourceString) {
        self.span_open(HURL_BASE64_VALUE_CLASS);
        self.push_source(source);
        self.span_close();
    }

    fn visit_bool(&mut self, value: bool) {
        self.span_open(HURL_BOOLEAN_CLASS);
        self.push_trusted(&value.to_string());
        self.span_close();
    }

    fn visit_cookie_path(&mut self, path: &CookiePath) {
        self.span_open(HURL_STRING_CLASS);
        self.push_source(&path.to_source());
        self.span_close();
    }

    fn visit_comment(&mut self, comment: &Comment) {
        self.span_open(HURL_COMMENT_CLASS);
        self.push_source(&comment.to_source());
        self.span_close();
    }

    fn visit_duration_unit(&mut self, unit: DurationUnit) {
        self.span_open(HURL_DURATION_UNIT);
        self.push_trusted(&unit.to_string());
        self.span_close();
    }

    fn visit_entry(&mut self, entry: &Entry) {
        self.span_open(HURL_ENTRY_CLASS);
        visit::walk_entry(self, entry);
        self.span_close();
    }

    fn visit_filename(&mut self, filename: &Template) {
        self.span_open(HURL_FILENAME_CLASS);
        self.push_source(&filename.to_source());
        self.span_close();
    }

    fn visit_filter_kind(&mut self, kind: &FilterValue) {
        self.span_open(HURL_FILTER_KIND_CLASS);
        self.push_trusted(kind.identifier());
        self.span_close();
    }

    fn visit_hex_value(&mut self, _value: &[u8], source: &SourceString) {
        self.span_open(HURL_HEX_CLASS);
        self.push_source(source);
        self.span_close();
    }

    fn visit_hurl_file(&mut self, file: &HurlFile) {
        self.pre_open(HURL_LANG_CLASS);
        visit::walk_hurl_file(self, file);
        self.pre_close();
    }

    fn visit_i64(&mut self, n: i64) {
        self.span_open(HURL_NUMBER_CLASS);
        self.push_trusted(&n.to_string());
        self.span_close();
    }

    fn visit_json_body(&mut self, json: &JsonValue) {
        self.span_open(HURL_JSON_CLASS);
        self.push_source(&json.to_source());
        self.span_close();
    }

    fn visit_literal(&mut self, lit: &'static str) {
        self.push_trusted(lit);
    }

    fn visit_method(&mut self, method: &Method) {
        self.span_open(HURL_METHOD_CLASS);
        self.push_trusted(&method.to_string());
        self.span_close();
    }

    fn visit_multiline_string(&mut self, string: &MultilineString) {
        self.span_open(HURL_MULTILINESTRING_CLASS);
        self.push_source(&string.to_source());
        self.span_close();
    }

    fn visit_not(&mut self, identifier: &'static str) {
        self.span_open(HURL_NOT_CLASS);
        self.push_trusted(identifier);
        self.span_close();
    }

    fn visit_null(&mut self, null: &'static str) {
        self.span_open(HURL_NULL_CLASS);
        self.push_trusted(null);
        self.span_close();
    }

    fn visit_number(&mut self, number: &Number) {
        self.span_open(HURL_NUMBER_CLASS);
        self.push_source(&number.to_source());
        self.span_close();
    }

    fn visit_placeholder(&mut self, placeholder: &Placeholder) {
        self.span_open(HURL_PLACEHOLDER_CLASS);
        self.push_source(&placeholder.to_source());
        self.span_close();
    }

    fn visit_predicate_kind(&mut self, kind: &PredicateFuncValue) {
        self.span_open(HURL_PREDICATE_TYPE_CLASS);
        self.push_source(&kind.to_source());
        self.span_close();
    }

    fn visit_query_kind(&mut self, kind: &QueryValue) {
        self.span_open(HURL_QUERY_TYPE_CLASS);
        self.push_trusted(kind.identifier());
        self.span_close();
    }

    fn visit_request(&mut self, request: &Request) {
        self.span_open(HURL_REQUEST_CLASS);
        visit::walk_request(self, request);
        self.span_close();
    }
    fn visit_response(&mut self, response: &Response) {
        self.span_open(HURL_RESPONSE_CLASS);
        visit::walk_response(self, response);
        self.span_close();
    }

    fn visit_regex(&mut self, regex: &Regex) {
        self.span_open(HURL_REGEX_CLASS);
        self.push_source(&regex.to_source());
        self.span_close();
    }

    fn visit_status(&mut self, value: &StatusValue) {
        self.span_open(HURL_NUMBER_CLASS);
        self.push_trusted(&value.to_string());
        self.span_close();
    }

    fn visit_string(&mut self, value: &str) {
        self.span_open(HURL_STRING_CLASS);
        self.push_untrusted(value);
        self.span_close();
    }

    fn visit_section_header(&mut self, name: &str) {
        self.span_open(HURL_SECTION_HEADER_CLASS);
        self.push_trusted(name);
        self.span_close();
    }

    fn visit_template(&mut self, template: &Template) {
        self.span_open(HURL_STRING_CLASS);
        self.push_source(&template.to_source());
        self.span_close();
    }

    fn visit_url(&mut self, url: &Template) {
        self.span_open(HURL_URL_CLASS);
        self.push_source(&url.to_source());
        self.span_close();
    }

    fn visit_u64(&mut self, n: &U64) {
        self.span_open(HURL_NUMBER_CLASS);
        self.push_trusted(n.to_source().as_str());
        self.span_close();
    }

    fn visit_usize(&mut self, n: usize) {
        self.span_open(HURL_NUMBER_CLASS);
        self.push_trusted(&n.to_string());
        self.span_close();
    }

    fn visit_variable_name(&mut self, name: &str) {
        self.push_trusted(name);
    }

    fn visit_version(&mut self, value: &VersionValue) {
        self.span_open(HURL_VERSION_CLASS);
        self.push_trusted(&value.to_string());
        self.span_close();
    }

    fn visit_xml_body(&mut self, xml: &str) {
        self.span_open(HURL_XML_CLASS);
        self.push_untrusted(xml);
        self.span_close();
    }

    fn visit_whitespace(&mut self, ws: &Whitespace) {
        self.push_trusted(ws.as_str());
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::visit::Visitor;
    use crate::ast::{
        JsonObjectElement, JsonValue, MultilineString, MultilineStringKind, SourceInfo, Template,
        TemplateElement, Whitespace,
    };
    use crate::format::html::HtmlFormatter;
    use crate::reader::Pos;
    use crate::types::ToSource;

    #[test]
    fn test_multiline_string() {
        // ```
        // line1
        // line2
        // ```
        let kind = MultilineStringKind::Text(Template {
            delimiter: None,
            elements: vec![TemplateElement::String {
                value: "line1\nline2\n".to_string(),
                source: "line1\nline2\n".to_source(),
            }],
            source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
        });
        let attributes = vec![];
        let multiline_string = MultilineString {
            attributes,
            space: Whitespace {
                value: String::new(),
                source_info: SourceInfo {
                    start: Pos { line: 1, column: 4 },
                    end: Pos { line: 1, column: 4 },
                },
            },
            newline: Whitespace {
                value: "\n".to_string(),
                source_info: SourceInfo {
                    start: Pos { line: 1, column: 4 },
                    end: Pos { line: 2, column: 1 },
                },
            },
            kind,
        };
        let mut fmt = HtmlFormatter::new();
        fmt.visit_multiline_string(&multiline_string);
        assert_eq!(
            fmt.buffer,
            "<span class=\"multiline\">```\nline1\nline2\n```</span>"
        );
    }

    #[test]
    fn test_json() {
        let value = JsonValue::Object {
            space0: String::new(),
            elements: vec![JsonObjectElement {
                space0: "\n   ".to_string(),
                name: Template::new(
                    Some('"'),
                    vec![TemplateElement::String {
                        value: "id".to_string(),
                        source: "id".to_source(),
                    }],
                    SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                ),
                space1: String::new(),
                space2: " ".to_string(),
                value: JsonValue::Number("1".to_string()),
                space3: "\n".to_string(),
            }],
        };
        let mut fmt = HtmlFormatter::new();
        fmt.visit_json_body(&value);
        assert_eq!(fmt.buffer, "<span class=\"json\">{\n   \"id\": 1\n}</span>");
    }

    #[test]
    fn test_json_encoded_newline() {
        let value = JsonValue::String(Template::new(
            Some('"'),
            vec![TemplateElement::String {
                value: "\n".to_string(),
                source: "\\n".to_source(),
            }],
            SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
        ));
        let mut fmt = HtmlFormatter::new();
        fmt.visit_json_body(&value);
        assert_eq!(fmt.buffer, "<span class=\"json\">\"\\n\"</span>");
    }

    #[test]
    fn test_xml() {
        let value = "<?xml version=\"1.0\"?>\n<drink>café</drink>";

        let mut fmt = HtmlFormatter::new();
        fmt.visit_xml_body(value);
        assert_eq!(
            fmt.buffer,
            "<span class=\"xml\">&lt;?xml version=\"1.0\"?&gt;\n&lt;drink&gt;café&lt;/drink&gt;</span>"
        );
    }

    #[test]
    fn test_xml_escape() {
        let mut fmt = HtmlFormatter::new();
        fmt.push_untrusted("hello");
        assert_eq!(fmt.buffer, "hello");

        let mut fmt = HtmlFormatter::new();
        fmt.push_untrusted("<?xml version=\"1.0\"?>");
        assert_eq!(fmt.buffer, "&lt;?xml version=\"1.0\"?&gt;");
    }
}
