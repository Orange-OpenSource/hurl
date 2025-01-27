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
use crate::ast::{Assert, Body, Bytes, Comment, CookiePath, Entry, Filter, FilterValue, HurlFile, KeyValue, LineTerminator, Method, Predicate, Query, QueryValue, Request, Response, Section, SectionValue, Template, Whitespace, U64};

trait Visitor: Sized {
    fn visit_assert(&mut self, assert: &Assert) {
        walk_assert(self, assert);
    }

    fn visit_body(&mut self, body: &Body) {
        walk_body(self, body)
    }

    fn visit_bytes(&mut self, bytes: &Bytes) {
        walk_bytes(self, bytes)
    }

    fn visit_cookie_path(&mut self, _path: &CookiePath) {}

    fn visit_comment(&mut self, _comment: &Comment) {}

    fn visit_entry(&mut self, entry: &Entry) {
        walk_entry(self, entry);
    }

    fn visit_filter(&mut self, filter: &Filter) {
        walk_filter(self, filter);
    }

    fn visit_filter_kind(&mut self, _kind: &FilterValue) {}

    fn visit_header(&mut self, header: &KeyValue) {
        walk_header(self, header);
    }

    fn visit_hurl_file(&mut self, hurl_file: &HurlFile) {
        walk_hurl_file(self, hurl_file);
    }

    fn visit_kv(&mut self, kv: &KeyValue) {
        walk_kv(self, kv);
    }

    fn visit_lt(&mut self, lt: &LineTerminator) {
        walk_lt(self, lt);
    }

    fn visit_literal(&mut self, _lit: &str) {}

    fn visit_method(&mut self, _method: &Method) {}

    fn visit_newline(&mut self, _nl: &Whitespace) {}

    fn visit_predicate(&mut self, predicate: &Predicate) {
        walk_predicate(self, predicate);
    }

    fn visit_query(&mut self, query: &Query) {
        walk_query(self, query);
    }

    fn visit_query_kind(&mut self, _kind: &QueryValue) {}

    fn visit_request(&mut self, request: &Request) {
        walk_request(self, request);
    }

    fn visit_response(&mut self, response: &Response) {
        walk_response(self, response);
    }

    fn visit_section(&mut self, section: &Section) {
        walk_section(self, section)
    }

    fn visit_section_header(&mut self, _name: &str) {}

    fn visit_section_value(&mut self, section_value: &SectionValue) {
        walk_section_value(self, section_value);
    }

    fn visit_template(&mut self, _template: &Template) {}

    fn visit_url(&mut self, _url: &Template) {}

    fn visit_u64(&mut self, _n: &U64) {}

    fn visit_whitespace(&mut self, _ws: &Whitespace) {}
}

fn walk_assert<V: Visitor>(visitor: &mut V, assert: &Assert) {
    assert
        .line_terminators
        .iter()
        .for_each(|lt| visitor.visit_lt(lt));
    visitor.visit_whitespace(&assert.space0);
    visitor.visit_query(&assert.query);
    for (space, filter) in assert.filters.iter() {
        visitor.visit_whitespace(space);
        visitor.visit_filter(filter);
    }
    visitor.visit_whitespace(&assert.space1);
    visitor.visit_predicate(&assert.predicate);
    visitor.visit_lt(&assert.line_terminator0);
}

fn walk_body<V: Visitor>(visitor: &mut V, body: &Body) {
    body.line_terminators.iter().for_each(|lt| visitor.visit_lt(lt));
    visitor.visit_whitespace(&body.space0);
    visitor.visit_bytes(&body.value);
    visitor.visit_lt(&body.line_terminator0);
}

fn walk_bytes<V: Visitor>(visitor: &mut V, bytes: &Bytes) {
    match bytes {
        Bytes::Json(value) => visitor.visit_json(value),
        Bytes::Xml(value) => visitor.visit_xml(value),
        Bytes::MultilineString(value) => visitor.visit_multiline_string_body(value),
        Bytes::OnelineString(value) => visitor.visit_template(value),
        Bytes::Base64(value) => visitor.visit_base64(value),
        Bytes::File(_) => {}
        Bytes::Hex(_) => {}
    }
}

fn walk_entry<V: Visitor>(visitor: &mut V, entry: &Entry) {
    visitor.visit_request(&entry.request);
    if let Some(ref response) = entry.response {
        visitor.visit_response(response);
    }
}

fn walk_filter<V: Visitor>(visitor: &mut V, filter: &Filter) {
    visitor.visit_filter_kind(&filter.value);
    match &filter.value {
        FilterValue::Base64Decode => {}
        FilterValue::Base64Encode => {}
        FilterValue::Count => {}
        FilterValue::DaysAfterNow => {}
        FilterValue::DaysBeforeNow => {}
        FilterValue::Decode { space0, encoding } => {
            visitor.visit_whitespace(space0);
            visitor.visit_template(encoding);
        }
        FilterValue::Format { space0, fmt } => {
            visitor.visit_whitespace(space0);
            visitor.visit_template(fmt);
        }
        FilterValue::HtmlEscape => {}
        FilterValue::HtmlUnescape => {}
        FilterValue::JsonPath { space0, expr } => {
            visitor.visit_whitespace(space0);
            visitor.visit_template(expr);
        }
        FilterValue::Nth { space0, n } => {
            visitor.visit_whitespace(space0);
            visitor.visit_u64(n);
        }
        FilterValue::Regex { .. } => {}
        FilterValue::Replace { .. } => {}
        FilterValue::Split { .. } => {}
        FilterValue::ToDate { .. } => {}
        FilterValue::ToFloat => {}
        FilterValue::ToInt => {}
        FilterValue::UrlDecode => {}
        FilterValue::UrlEncode => {}
        FilterValue::XPath { .. } => {}
    }
}

fn walk_header<V: Visitor>(visitor: &mut V, header: &KeyValue) {
    visitor.visit_kv(header)
}

fn walk_hurl_file<V: Visitor>(visitor: &mut V, hurl_file: &HurlFile) {
    hurl_file
        .entries
        .iter()
        .for_each(|e| visitor.visit_entry(e));
    hurl_file
        .line_terminators
        .iter()
        .for_each(|lt| visitor.visit_lt(lt));
}

fn walk_kv<V: Visitor>(visitor: &mut V, kv: &KeyValue) {
    kv.line_terminators
        .iter()
        .for_each(|lt| visitor.visit_lt(lt));
    visitor.visit_whitespace(&kv.space0);
    visitor.visit_template(&kv.key);
    visitor.visit_whitespace(&kv.space1);
    visitor.visit_colon();
    visitor.visit_whitespace(&kv.space2);
    visitor.visit_template(&kv.value);
    visitor.visit_lt(&kv.line_terminator0)
}

fn walk_lt<V: Visitor>(visitor: &mut V, lt: &LineTerminator) {
    visitor.visit_whitespace(&lt.space0);
    if let Some(ref comment) = lt.comment {
        visitor.visit_comment(comment);
    }
    visitor.visit_newline(&lt.newline);
}

fn walk_query<V: Visitor>(visitor: &mut V, query: &Query) {
    visitor.visit_query_kind(&query.value);

    match &query.value {
        QueryValue::Header { space0, name } => {
            visitor.visit_whitespace(space0);
            visitor.visit_template(name);
        }
        QueryValue::Cookie { space0, expr } => {
            visitor.visit_whitespace(space0);
            visitor.visit_cookie_path(expr);
        }
        QueryValue::Xpath { space0, expr } => {
            visitor.visit_whitespace(space0);
            visitor.visit_template(expr);
        }
        QueryValue::Jsonpath { space0, expr } => {
            visitor.visit_whitespace(space0);
            visitor.visit_template(expr);
        }
        QueryValue::Regex { space0, value } => {
            visitor.visit_whitespace(space0);
            visitor.visit_regex(value);
        }
        QueryValue::Variable { space0, name } => {
            visitor.visit_whitespace(space0);
            visitor.visit_template(name);
        }
        QueryValue::Certificate {
            space0,
            attribute_name,
        } => {
            visitor.visit_whitespace(space0);
            visitor.visit_certificate_attribute_name(attribute_name);
        }
        QueryValue::Body
        | QueryValue::Status
        | QueryValue::Url
        | QueryValue::Duration
        | QueryValue::Bytes
        | QueryValue::Sha256
        | QueryValue::Md5 => {}
    }
}

fn walk_predicate<V: Visitor>(visitor: &mut V, pred: &Predicate) {
    if pred.not {
        visitor.visit_not();
        visitor.visit_whitespace(&pred.space0);
    }
    visitor.visit_predicate_func(&pred.predicate_func);
}

fn walk_request<V: Visitor>(visitor: &mut V, request: &Request) {
    request
        .line_terminators
        .iter()
        .for_each(|lt| visitor.visit_lt(lt));
    visitor.visit_whitespace(&request.space0);
    visitor.visit_method(&request.method);
    visitor.visit_whitespace(&request.space1);
    visitor.visit_url(&request.url);
    visitor.visit_lt(&request.line_terminator0);
    request.headers.iter().for_each(|h| visitor.visit_kv(h));
    request
        .sections
        .iter()
        .for_each(|s| visitor.visit_section(s));
}

fn walk_response<V: Visitor>(visitor: &mut V, response: &Response) {
    response
        .line_terminators
        .iter()
        .for_each(|lt| visitor.visit_lt(lt));
    visitor.visit_whitespace(&response.space0);
    visitor.visit_whitespace(&response.space1);
    visitor.visit_lt(&response.line_terminator0);
}

fn walk_section<V: Visitor>(visitor: &mut V, section: &Section) {
    section
        .line_terminators
        .iter()
        .for_each(|lt| visitor.visit_lt(lt));
    visitor.visit_whitespace(&section.space0);
    visitor.visit_section_header(&section.name());
    visitor.visit_lt(&section.line_terminator0);
    visitor.visit_section_value(&section.value);
}

fn walk_section_value<V: Visitor>(visitor: &mut V, section_value: &SectionValue) {
    match section_value {
        SectionValue::Asserts(asserts) => asserts.iter().for_each(|a| visitor.visit_assert(a)),
        SectionValue::BasicAuth(Some(auth)) => visitor.visit_kv(auth),
        SectionValue::BasicAuth(_) => {}
        SectionValue::Captures(captures) => captures.iter().for_each(|c| visitor.visit_capture(c)),
        SectionValue::Cookies(cookies) => cookies.iter().for_each(|c| visitor.visit_cookie(c)),
        SectionValue::FormParams(params, _) => params.iter().for_each(|p| visitor.visit_kv(p)),
        SectionValue::MultipartFormData(params, _) => {
            params.iter().for_each(|p| visitor.visit_multipart(p))
        }
        SectionValue::Options(options) => {
            options.iter().for_each(|o| visitor.visit_entry_option(o))
        }
        SectionValue::QueryParams(params, _) => params.iter().for_each(|p| visitor.visit_kv(p)),
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::visit::{walk_entry, walk_hurl_file, walk_request, Visitor};
    use crate::ast::{Comment, Entry, HurlFile, Method, Request, Template, Whitespace};
    use crate::parser;

    struct HtmlFormatter {
        buffer: String,
    }

    fn escape_xml(s: &str) -> String {
        s.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
    }

    impl HtmlFormatter {
        pub fn new() -> Self {
            HtmlFormatter {
                buffer: String::new(),
            }
        }

        pub fn format(&mut self, hurl_file: &HurlFile) -> &str {
            self.buffer.clear();
            self.visit_hurl_file(&hurl_file);
            &self.buffer
        }

        fn pre_open(&mut self, class: &str) {
            self.buffer.push_str("<pre><code class=\"");
            self.buffer.push_str(class);
            self.buffer.push_str("\">");
        }

        fn pre_close(&mut self) {
            self.buffer.push_str("</code></pre>");
        }

        fn span_open(&mut self, class: &str) {
            self.buffer.push_str("<span class=\"");
            self.buffer.push_str(class);
            self.buffer.push_str("\">");
        }

        fn span_close(&mut self) {
            self.buffer.push_str("</span>");
        }

        fn span(&mut self, class: &str, value: &str) {
            self.buffer.push_str("<span class=\"");
            self.buffer.push_str(class);
            self.buffer.push_str("\">");
            self.buffer.push_str(value);
            self.buffer.push_str("</span>");
        }
    }

    impl Visitor for HtmlFormatter {
        fn visit_hurl_file(&mut self, hurl_file: &HurlFile) {
            self.pre_open("language-hurl");
            walk_hurl_file(self, hurl_file);
            self.pre_close();
        }

        fn visit_entry(&mut self, entry: &Entry) {
            self.span_open("hurl-entry");
            walk_entry(self, entry);
            self.span_close();
        }

        fn visit_request(&mut self, request: &Request) {
            self.span_open("request");
            walk_request(self, request);
            self.span_close();
        }

        fn visit_method(&mut self, method: &Method) {
            self.span("method", &method.to_string());
        }

        fn visit_url(&mut self, url: &Template) {
            let url = escape_xml(&url.to_encoded_string());
            self.span("url", &url);
        }

        fn visit_comment(&mut self, comment: &Comment) {
            let comment = format!("#{}", escape_xml(&comment.value));
            self.span("comment", &comment);
        }

        fn visit_whitespace(&mut self, ws: &Whitespace) {
            self.buffer.push_str(&ws.value);
        }

        fn visit_newline(&mut self, _nl: &Whitespace) {
            self.buffer.push_str("<br />")
        }

        fn visit_literal(&mut self, lit: &str) {
            self.buffer.push_str(lit);
        }
    }

    #[test]
    fn format_to_html() {
        let txt = "# This is a comment
GET https://foo.com
HTTP 200
";
        let hurl_file = parser::parse_hurl_file(txt).unwrap();
        let mut html_formatter = HtmlFormatter::new();
        let html = html_formatter.format(&hurl_file);
        eprintln!("======");
        eprintln!("{txt}");
        eprintln!("======");
        eprintln!("{}", html);
    }
}
