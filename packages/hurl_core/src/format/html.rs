/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2023 Orange
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
use crate::ast::*;
use std::fmt::Display;

/// Returns an HTML string of the Hurl file `hurl_file`.
///
/// If `standalone` is true, a complete HTML body with inline styling is returned.
/// Otherwise, a `<pre>` HTML tag is returned, without styling.
pub fn format(hurl_file: &HurlFile, standalone: bool) -> String {
    let mut fmt = HtmlFormatter::new();
    let body = fmt.fmt_hurl_file(hurl_file);
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

impl HtmlFormatter {
    pub fn new() -> Self {
        HtmlFormatter {
            buffer: String::new(),
        }
    }

    pub fn fmt_hurl_file(&mut self, hurl_file: &HurlFile) -> &str {
        self.buffer.clear();
        self.fmt_pre_open("language-hurl");
        hurl_file.entries.iter().for_each(|e| self.fmt_entry(e));
        self.fmt_lts(&hurl_file.line_terminators);
        self.fmt_pre_close();
        &self.buffer
    }

    fn fmt_pre_open(&mut self, class: &str) {
        self.buffer.push_str("<pre><code class=\"");
        self.buffer.push_str(class);
        self.buffer.push_str("\">");
    }

    fn fmt_pre_close(&mut self) {
        self.buffer.push_str("</code></pre>");
    }

    fn fmt_span_open(&mut self, class: &str) {
        self.buffer.push_str("<span class=\"");
        self.buffer.push_str(class);
        self.buffer.push_str("\">");
    }

    fn fmt_span_close(&mut self) {
        self.buffer.push_str("</span>");
    }

    fn fmt_span(&mut self, class: &str, value: &str) {
        self.buffer.push_str("<span class=\"");
        self.buffer.push_str(class);
        self.buffer.push_str("\">");
        self.buffer.push_str(value);
        self.buffer.push_str("</span>");
    }

    fn fmt_entry(&mut self, entry: &Entry) {
        self.fmt_span_open("hurl-entry");
        self.fmt_request(&entry.request);
        if let Some(response) = &entry.response {
            self.fmt_response(response);
        }
        self.fmt_span_close();
    }

    fn fmt_request(&mut self, request: &Request) {
        self.fmt_span_open("request");
        self.fmt_lts(&request.line_terminators);
        self.fmt_span_open("line");
        self.fmt_space(&request.space0);
        self.fmt_method(&request.method);
        self.fmt_space(&request.space1);
        let url = escape_xml(&request.url.to_encoded_string());
        self.fmt_span("url", &url);
        self.fmt_span_close();
        self.fmt_lt(&request.line_terminator0);
        request.headers.iter().for_each(|h| self.fmt_kv(h));
        request.sections.iter().for_each(|s| self.fmt_section(s));
        if let Some(body) = &request.body {
            self.fmt_body(body);
        }
        self.fmt_span_close();
    }

    fn fmt_response(&mut self, response: &Response) {
        self.fmt_span_open("response");
        self.fmt_lts(&response.line_terminators);
        self.fmt_span_open("line");
        self.fmt_space(&response.space0);
        self.fmt_version(&response.version);
        self.fmt_space(&response.space1);
        self.fmt_status(&response.status);
        self.fmt_span_close();
        self.fmt_lt(&response.line_terminator0);
        response.headers.iter().for_each(|h| self.fmt_kv(h));
        response.sections.iter().for_each(|s| self.fmt_section(s));
        if let Some(body) = &response.body {
            self.fmt_body(body);
        }
        self.fmt_span_close();
    }

    fn fmt_method(&mut self, method: &Method) {
        self.fmt_span("method", &method.to_string());
    }

    fn fmt_version(&mut self, version: &Version) {
        self.fmt_span("version", &version.value.to_string());
    }

    fn fmt_status(&mut self, status: &Status) {
        self.fmt_number(&status.value.to_string());
    }

    fn fmt_section(&mut self, section: &Section) {
        self.fmt_lts(&section.line_terminators);
        self.fmt_space(&section.space0);
        self.fmt_span_open("line");
        let name = format!("[{}]", section.name());
        self.fmt_span("section-header", &name);
        self.fmt_span_close();
        self.fmt_lt(&section.line_terminator0);
        self.fmt_section_value(&section.value);
    }

    fn fmt_section_value(&mut self, section_value: &SectionValue) {
        match section_value {
            SectionValue::Asserts(items) => items.iter().for_each(|item| self.fmt_assert(item)),
            SectionValue::QueryParams(items) => items.iter().for_each(|item| self.fmt_kv(item)),
            SectionValue::BasicAuth(item) => self.fmt_kv(item),
            SectionValue::FormParams(items) => items.iter().for_each(|item| self.fmt_kv(item)),
            SectionValue::MultipartFormData(items) => {
                items.iter().for_each(|item| self.fmt_multipart_param(item))
            }
            SectionValue::Cookies(items) => items.iter().for_each(|item| self.fmt_cookie(item)),
            SectionValue::Captures(items) => items.iter().for_each(|item| self.fmt_capture(item)),
            SectionValue::Options(items) => {
                items.iter().for_each(|item| self.fmt_entry_option(item))
            }
        }
    }

    fn fmt_kv(&mut self, kv: &KeyValue) {
        self.fmt_lts(&kv.line_terminators);
        self.fmt_span_open("line");
        self.fmt_space(&kv.space0);
        self.fmt_string(&kv.key.encoded);
        self.fmt_space(&kv.space1);
        self.buffer.push(':');
        self.fmt_space(&kv.space2);
        self.fmt_template(&kv.value);
        self.fmt_span_close();
        self.fmt_lt(&kv.line_terminator0);
    }

    fn fmt_entry_option(&mut self, entry_option: &EntryOption) {
        match entry_option {
            EntryOption::CaCertificate(option) => self.fmt_ca_certificate_option(option),
            EntryOption::ClientCert(option) => self.fmt_client_cert_option(option),
            EntryOption::ClientKey(option) => self.fmt_client_key_option(option),
            EntryOption::Compressed(option) => self.fmt_compressed_option(option),
            EntryOption::Insecure(option) => self.fmt_insecure_option(option),
            EntryOption::FollowLocation(option) => self.fmt_follow_location_option(option),
            EntryOption::MaxRedirect(option) => self.fmt_max_redirect_option(option),
            EntryOption::PathAsIs(option) => self.fmt_path_as_is_option(option),
            EntryOption::Proxy(option) => self.fmt_proxy_option(option),
            EntryOption::Retry(option) => self.fmt_retry_option(option),
            EntryOption::RetryInterval(option) => self.fmt_retry_interval_option(option),
            EntryOption::Variable(option) => self.fmt_variable_option(option),
            EntryOption::Verbose(option) => self.fmt_verbose_option(option),
            EntryOption::VeryVerbose(option) => self.fmt_very_verbose_option(option),
        };
    }

    fn fmt_compressed_option(&mut self, option: &CompressedOption) {
        self.fmt_lts(&option.line_terminators);
        self.fmt_span_open("line");
        self.fmt_space(&option.space0);
        self.fmt_string("compressed");
        self.fmt_space(&option.space1);
        self.buffer.push(':');
        self.fmt_space(&option.space2);
        self.fmt_bool(option.value);
        self.fmt_span_close();
        self.fmt_lt(&option.line_terminator0);
    }

    fn fmt_insecure_option(&mut self, option: &InsecureOption) {
        self.fmt_lts(&option.line_terminators);
        self.fmt_span_open("line");
        self.fmt_space(&option.space0);
        self.fmt_string("insecure");
        self.fmt_space(&option.space1);
        self.buffer.push(':');
        self.fmt_space(&option.space2);
        self.fmt_bool(option.value);
        self.fmt_span_close();
        self.fmt_lt(&option.line_terminator0);
    }

    fn fmt_ca_certificate_option(&mut self, option: &CaCertificateOption) {
        self.fmt_lts(&option.line_terminators);
        self.fmt_span_open("line");
        self.fmt_space(&option.space0);
        self.fmt_string("cacert");
        self.fmt_space(&option.space1);
        self.buffer.push(':');
        self.fmt_space(&option.space2);
        self.fmt_filename(&option.filename);
        self.fmt_span_close();
        self.fmt_lt(&option.line_terminator0);
    }

    fn fmt_client_cert_option(&mut self, option: &ClientCertOption) {
        self.fmt_lts(&option.line_terminators);
        self.fmt_span_open("line");
        self.fmt_space(&option.space0);
        self.fmt_string("cert");
        self.fmt_space(&option.space1);
        self.buffer.push(':');
        self.fmt_space(&option.space2);
        self.fmt_filename(&option.filename);
        self.fmt_span_close();
        self.fmt_lt(&option.line_terminator0);
    }

    fn fmt_client_key_option(&mut self, option: &ClientKeyOption) {
        self.fmt_lts(&option.line_terminators);
        self.fmt_span_open("line");
        self.fmt_space(&option.space0);
        self.fmt_string("key");
        self.fmt_space(&option.space1);
        self.buffer.push(':');
        self.fmt_space(&option.space2);
        self.fmt_filename(&option.filename);
        self.fmt_span_close();
        self.fmt_lt(&option.line_terminator0);
    }

    fn fmt_follow_location_option(&mut self, option: &FollowLocationOption) {
        self.fmt_lts(&option.line_terminators);
        self.fmt_span_open("line");
        self.fmt_space(&option.space0);
        self.fmt_string("location");
        self.fmt_space(&option.space1);
        self.buffer.push(':');
        self.fmt_space(&option.space2);
        self.fmt_bool(option.value);
        self.fmt_span_close();
        self.fmt_lt(&option.line_terminator0);
    }

    fn fmt_max_redirect_option(&mut self, option: &MaxRedirectOption) {
        self.fmt_lts(&option.line_terminators);
        self.fmt_span_open("line");
        self.fmt_space(&option.space0);
        self.fmt_string("max-redirs");
        self.fmt_space(&option.space1);
        self.buffer.push(':');
        self.fmt_space(&option.space2);
        self.fmt_number(option.value);
        self.fmt_span_close();
        self.fmt_lt(&option.line_terminator0);
    }

    fn fmt_path_as_is_option(&mut self, option: &PathAsIsOption) {
        self.fmt_lts(&option.line_terminators);
        self.fmt_span_open("line");
        self.fmt_space(&option.space0);
        self.fmt_string("path-as-is");
        self.fmt_space(&option.space1);
        self.buffer.push(':');
        self.fmt_space(&option.space2);
        self.fmt_bool(option.value);
        self.fmt_span_close();
        self.fmt_lt(&option.line_terminator0);
    }

    fn fmt_proxy_option(&mut self, option: &ProxyOption) {
        self.fmt_lts(&option.line_terminators);
        self.fmt_span_open("line");
        self.fmt_space(&option.space0);
        self.fmt_string("proxy");
        self.fmt_space(&option.space1);
        self.buffer.push(':');
        self.fmt_space(&option.space2);
        self.fmt_string(&option.value);
        self.fmt_span_close();
        self.fmt_lt(&option.line_terminator0);
    }

    fn fmt_retry_option(&mut self, option: &RetryOption) {
        self.fmt_lts(&option.line_terminators);
        self.fmt_span_open("line");
        self.fmt_space(&option.space0);
        self.fmt_string("retry");
        self.fmt_space(&option.space1);
        self.buffer.push(':');
        self.fmt_space(&option.space2);
        self.fmt_retry(option.value);
        self.fmt_span_close();
        self.fmt_lt(&option.line_terminator0);
    }

    fn fmt_retry_interval_option(&mut self, option: &RetryIntervalOption) {
        self.fmt_lts(&option.line_terminators);
        self.fmt_span_open("line");
        self.fmt_space(&option.space0);
        self.fmt_string("retry-interval");
        self.fmt_space(&option.space1);
        self.buffer.push(':');
        self.fmt_space(&option.space2);
        self.fmt_number(option.value);
        self.fmt_span_close();
        self.fmt_lt(&option.line_terminator0);
    }

    fn fmt_variable_option(&mut self, option: &VariableOption) {
        self.fmt_lts(&option.line_terminators);
        self.fmt_span_open("line");
        self.fmt_space(&option.space0);
        self.fmt_string("variable");
        self.fmt_space(&option.space1);
        self.buffer.push(':');
        self.fmt_space(&option.space2);
        self.fmt_variable_definition(&option.value);
        self.fmt_span_close();
        self.fmt_lt(&option.line_terminator0);
    }

    fn fmt_variable_definition(&mut self, option: &VariableDefinition) {
        self.buffer.push_str(option.name.as_str());
        self.fmt_space(&option.space1);
        self.buffer.push('=');
        self.fmt_variable_value(&option.value);
    }

    fn fmt_variable_value(&mut self, option: &VariableValue) {
        match option {
            VariableValue::Null { .. } => self.fmt_span("null", "null"),
            VariableValue::Bool(v) => self.fmt_bool(*v),
            VariableValue::Integer(v) => self.fmt_number(v),
            VariableValue::Float(v) => self.fmt_number(&v.encoded),
            VariableValue::String(t) => self.fmt_template(t),
        }
    }

    fn fmt_verbose_option(&mut self, option: &VerboseOption) {
        self.fmt_lts(&option.line_terminators);
        self.fmt_span_open("line");
        self.fmt_space(&option.space0);
        self.fmt_string("verbose");
        self.fmt_space(&option.space1);
        self.buffer.push(':');
        self.fmt_space(&option.space2);
        self.fmt_bool(option.value);
        self.fmt_span_close();
        self.fmt_lt(&option.line_terminator0);
    }

    fn fmt_very_verbose_option(&mut self, option: &VeryVerboseOption) {
        self.fmt_lts(&option.line_terminators);
        self.fmt_span_open("line");
        self.fmt_space(&option.space0);
        self.fmt_string("very-verbose");
        self.fmt_space(&option.space1);
        self.buffer.push(':');
        self.fmt_space(&option.space2);
        self.fmt_bool(option.value);
        self.fmt_span_close();
        self.fmt_lt(&option.line_terminator0);
    }

    fn fmt_multipart_param(&mut self, param: &MultipartParam) {
        match param {
            MultipartParam::Param(param) => self.fmt_kv(param),
            MultipartParam::FileParam(param) => self.fmt_file_param(param),
        };
    }

    fn fmt_file_param(&mut self, param: &FileParam) {
        self.fmt_lts(&param.line_terminators);
        self.fmt_span_open("line");
        self.fmt_space(&param.space0);
        self.fmt_string(&param.key.encoded);
        self.fmt_space(&param.space1);
        self.buffer.push(':');
        self.fmt_space(&param.space2);
        self.fmt_file_value(&param.value);
        self.fmt_span_close();
        self.fmt_lt(&param.line_terminator0);
    }

    fn fmt_file_value(&mut self, file_value: &FileValue) {
        self.buffer.push_str("file,");
        self.fmt_space(&file_value.space0);
        self.fmt_filename(&file_value.filename);
        self.fmt_space(&file_value.space1);
        self.buffer.push(';');
        self.fmt_space(&file_value.space2);
        if let Some(content_type) = &file_value.content_type {
            self.fmt_string(content_type);
        }
    }

    fn fmt_filename(&mut self, filename: &Filename) {
        self.fmt_span_open("filename");
        let s = filename.value.replace(' ', "\\ ");
        self.buffer.push_str(s.as_str());
        self.fmt_span_close();
    }

    fn fmt_cookie(&mut self, cookie: &Cookie) {
        self.fmt_lts(&cookie.line_terminators);
        self.fmt_span_open("line");
        self.fmt_space(&cookie.space0);
        self.fmt_span("name", &cookie.name.value);
        self.fmt_space(&cookie.space1);
        self.buffer.push(':');
        self.fmt_space(&cookie.space2);
        self.fmt_template(&cookie.value);
        self.fmt_span_close();
        self.fmt_lt(&cookie.line_terminator0);
    }

    fn fmt_capture(&mut self, capture: &Capture) {
        self.fmt_lts(&capture.line_terminators);
        self.fmt_span_open("line");
        self.fmt_space(&capture.space0);
        self.fmt_span("name", &capture.name.value);
        self.fmt_space(&capture.space1);
        self.buffer.push(':');
        self.fmt_space(&capture.space2);
        self.fmt_query(&capture.query);
        for (space, filter) in capture.filters.iter() {
            self.fmt_space(space);
            self.fmt_filter(filter);
        }
        self.fmt_span_close();
        self.fmt_lt(&capture.line_terminator0);
    }

    fn fmt_query(&mut self, query: &Query) {
        self.fmt_query_value(&query.value);
    }

    fn fmt_query_value(&mut self, query_value: &QueryValue) {
        match query_value {
            QueryValue::Status {} => self.fmt_span("query-type", "status"),
            QueryValue::Url {} => self.fmt_span("query-type", "url"),
            QueryValue::Header { space0, name } => {
                self.fmt_span("query-type", "header");
                self.fmt_space(space0);
                self.fmt_template(name);
            }
            QueryValue::Cookie { space0, expr } => {
                self.fmt_span("query-type", "cookie");
                self.fmt_space(space0);
                self.fmt_cookie_path(expr);
            }
            QueryValue::Body {} => self.fmt_span("query-type", "body"),
            QueryValue::Xpath { space0, expr } => {
                self.fmt_span("query-type", "xpath");
                self.fmt_space(space0);
                self.fmt_template(expr);
            }
            QueryValue::Jsonpath { space0, expr } => {
                self.fmt_span("query-type", "jsonpath");
                self.fmt_space(space0);
                self.fmt_template(expr);
            }
            QueryValue::Regex { space0, value } => {
                self.fmt_span("query-type", "regex");
                self.fmt_space(space0);
                self.fmt_regex_value(value);
            }
            QueryValue::Variable { space0, name } => {
                self.fmt_span("query-type", "variable");
                self.fmt_space(space0);
                self.fmt_template(name);
            }
            QueryValue::Duration {} => self.fmt_span("query-type", "duration"),
            QueryValue::Bytes {} => self.fmt_span("query-type", "bytes"),
            QueryValue::Sha256 {} => self.fmt_span("query-type", "sha256"),
            QueryValue::Md5 {} => self.fmt_span("query-type", "md5"),
            QueryValue::Certificate {
                space0,
                attribute_name: field,
            } => {
                self.fmt_span("query-type", "certificate");
                self.fmt_space(space0);
                self.fmt_certificate_attribute_name(field);
            }
        }
    }

    fn fmt_regex_value(&mut self, regex_value: &RegexValue) {
        match regex_value {
            RegexValue::Template(template) => self.fmt_template(template),
            RegexValue::Regex(regex) => self.fmt_regex(regex),
        }
    }

    fn fmt_cookie_path(&mut self, cookie_path: &CookiePath) {
        self.fmt_span_open("string");
        self.buffer.push('"');
        self.buffer
            .push_str(cookie_path.name.to_encoded_string().as_str());
        if let Some(attribute) = &cookie_path.attribute {
            self.buffer.push('[');
            self.fmt_cookie_attribute(attribute);
            self.buffer.push(']');
        }
        self.buffer.push('"');
        self.fmt_span_close();
    }

    fn fmt_cookie_attribute(&mut self, cookie_attribute: &CookieAttribute) {
        self.fmt_space(&cookie_attribute.space0);
        self.buffer.push_str(cookie_attribute.name.value().as_str());
        self.fmt_space(&cookie_attribute.space1);
    }

    fn fmt_certificate_attribute_name(&mut self, name: &CertificateAttributeName) {
        let value = match name {
            CertificateAttributeName::Subject => "Subject",
            CertificateAttributeName::Issuer => "Issuer",
            CertificateAttributeName::StartDate => "Start-Date",
            CertificateAttributeName::ExpireDate => "Expire-Date",
            CertificateAttributeName::SerialNumber => "Serial-Number",
        };
        self.fmt_span_open("string");
        self.buffer.push('"');
        self.buffer.push_str(value);
        self.buffer.push('"');
        self.fmt_span_close();
    }

    fn fmt_assert(&mut self, assert: &Assert) {
        self.fmt_lts(&assert.line_terminators);
        self.fmt_span_open("line");
        self.fmt_space(&assert.space0);
        self.fmt_query(&assert.query);
        for (space, filter) in assert.filters.iter() {
            self.fmt_space(space);
            self.fmt_filter(filter);
        }
        self.fmt_space(&assert.space1);
        self.fmt_predicate(&assert.predicate);
        self.fmt_span_close();
        self.fmt_lt(&assert.line_terminator0);
    }

    fn fmt_predicate(&mut self, predicate: &Predicate) {
        if predicate.not {
            self.fmt_span("not", "not");
            self.fmt_space(&predicate.space0);
        }
        self.fmt_predicate_func(&predicate.predicate_func);
    }

    fn fmt_predicate_func(&mut self, predicate_func: &PredicateFunc) {
        self.fmt_predicate_func_value(&predicate_func.value);
    }

    fn fmt_predicate_func_value(&mut self, value: &PredicateFuncValue) {
        self.fmt_span_open("predicate-type");
        self.buffer.push_str(&encode_html(value.name()));
        self.fmt_span_close();

        match value {
            PredicateFuncValue::CountEqual { space0, value, .. } => {
                self.fmt_space(space0);
                self.fmt_predicate_value(value);
            }
            PredicateFuncValue::Equal { space0, value, .. } => {
                self.fmt_space(space0);
                self.fmt_predicate_value(value);
            }
            PredicateFuncValue::NotEqual { space0, value, .. } => {
                self.fmt_space(space0);
                self.fmt_predicate_value(value);
            }
            PredicateFuncValue::GreaterThan { space0, value, .. } => {
                self.fmt_space(space0);
                self.fmt_predicate_value(value);
            }
            PredicateFuncValue::GreaterThanOrEqual { space0, value, .. } => {
                self.fmt_space(space0);
                self.fmt_predicate_value(value);
            }
            PredicateFuncValue::LessThan { space0, value, .. } => {
                self.fmt_space(space0);
                self.fmt_predicate_value(value);
            }
            PredicateFuncValue::LessThanOrEqual { space0, value, .. } => {
                self.fmt_space(space0);
                self.fmt_predicate_value(value);
            }
            PredicateFuncValue::StartWith { space0, value } => {
                self.fmt_space(space0);
                self.fmt_predicate_value(value);
            }
            PredicateFuncValue::EndWith { space0, value } => {
                self.fmt_space(space0);
                self.fmt_predicate_value(value);
            }
            PredicateFuncValue::Contain { space0, value } => {
                self.fmt_space(space0);
                self.fmt_predicate_value(value);
            }
            PredicateFuncValue::Include { space0, value } => {
                self.fmt_space(space0);
                self.fmt_predicate_value(value);
            }
            PredicateFuncValue::Match { space0, value } => {
                self.fmt_space(space0);
                self.fmt_predicate_value(value);
            }
            PredicateFuncValue::IsInteger {} => {}
            PredicateFuncValue::IsFloat {} => {}
            PredicateFuncValue::IsBoolean {} => {}
            PredicateFuncValue::IsString {} => {}
            PredicateFuncValue::IsCollection {} => {}
            PredicateFuncValue::Exist {} => {}
            PredicateFuncValue::IsEmpty {} => {}
        }
    }

    fn fmt_predicate_value(&mut self, predicate_value: &PredicateValue) {
        match predicate_value {
            PredicateValue::String(value) => self.fmt_template(value),
            PredicateValue::MultilineString(value) => self.fmt_multiline_string(value, false),
            PredicateValue::Integer(value) => self.fmt_number(value),
            PredicateValue::Float(value) => self.fmt_number(&value.encoded),
            PredicateValue::Bool(value) => self.fmt_bool(*value),
            PredicateValue::Hex(value) => self.fmt_hex(value),
            PredicateValue::Base64(value) => self.fmt_base64(value),
            PredicateValue::Expression(value) => self.fmt_expr(value),
            PredicateValue::Null {} => self.fmt_span("null", "null"),
            PredicateValue::Regex(value) => self.fmt_regex(value),
        };
    }

    fn fmt_multiline_string(&mut self, multiline_string: &MultilineString, as_body: bool) {
        if let MultilineString::OneLineText(line) = multiline_string {
            let line = format!("```{line}```");
            if as_body {
                self.fmt_span_open("multiline");
                self.fmt_span("line", &line);
                self.fmt_span_close();
            } else {
                self.fmt_span("multiline", &line);
            }
            return;
        }

        // The multiline spans multiple newlines. We distinguish cases for multiline
        // as a body and multiline as a predicate value. When used as a body, we can embed
        // span lines with the multiline span. Used as a predicate, we have to break the multiline
        // span in two parts.
        //
        // # Case 1: multiline string as a body
        //
        // ~~~hurl
        // GET https://foo.com
        // ```
        // line1
        // line2
        // line3
        // ```
        // ~~~
        //
        // We embed span lines inside the span for the body:
        //
        // ```html
        // ...
        // <span class="multiline">
        //   <span class="line">```</span>
        //   <span class="line">line1</span>
        //   <span class="line">line2</span>
        //   <span class="line">line3</span>
        //   <span class="line">```</span>
        // </span>
        // ```
        //
        // # Case 1: multiline string as a predicate value
        //
        // ~~~hurl
        // GET https://foo.com
        // HTTP 200
        // [Asserts]
        // body == ```
        // line1
        // line2
        // line3
        // ```
        // ~~~
        //
        // ```html
        // ...
        // <span class="line">body ==
        //   <span class="multiline">```</span>
        // </span>
        // <span class="multiline">
        //   <span class="line">line1</span>
        //   <span class="line">line2</span>
        //   <span class="line">line3</span>
        //   <span class="line">```</span>
        // </span>
        // ```
        let lang = multiline_string.lang();
        if as_body {
            let body = format!("```{lang}\n{multiline_string}```");
            let body = format_multilines(&body);
            self.fmt_span("multiline", &body);
        } else {
            let head = format!("```{lang}");
            self.fmt_span("multiline", &head);
            // We close the current span line opened by the assert
            self.fmt_span_close();
            self.buffer.push('\n');
            let tail = format!("{multiline_string}```");
            let tail = format_multilines(&tail);
            self.fmt_span("multiline", &tail);
            // As we have added a span close, we must remove one to have the right number
            // of span. The current span line will add a closing span.
            pop_str(&mut self.buffer, "</span>")
        }
    }

    fn fmt_body(&mut self, body: &Body) {
        self.fmt_lts(&body.line_terminators);
        self.fmt_space(&body.space0);
        self.fmt_bytes(&body.value);
        self.fmt_lt(&body.line_terminator0);
    }

    fn fmt_bytes(&mut self, bytes: &Bytes) {
        match bytes {
            Bytes::Base64(value) => {
                self.fmt_span_open("line");
                self.fmt_base64(value);
                self.fmt_span_close();
            }
            Bytes::File(value) => {
                self.fmt_span_open("line");
                self.fmt_file(value);
                self.fmt_span_close();
            }
            Bytes::Hex(value) => {
                self.fmt_span_open("line");
                self.fmt_hex(value);
                self.fmt_span_close();
            }
            Bytes::OnelineString(value) => {
                self.fmt_span_open("line");
                self.fmt_template(value);
                self.fmt_span_close();
            }
            Bytes::Json(value) => self.fmt_json_value(value),
            Bytes::MultilineString(value) => self.fmt_multiline_string(value, true),
            Bytes::Xml(value) => self.fmt_xml(value),
        }
    }

    fn fmt_string(&mut self, value: &str) {
        self.fmt_span("string", value);
    }

    fn fmt_bool(&mut self, value: bool) {
        self.fmt_span("boolean", &value.to_string());
    }

    fn fmt_number<T: Sized + Display>(&mut self, value: T) {
        self.fmt_span("number", &value.to_string());
    }

    fn fmt_xml(&mut self, value: &str) {
        let xml = format_multilines(value);
        self.fmt_span("xml", &xml);
    }

    fn fmt_json_value(&mut self, json_value: &JsonValue) {
        let json = format_multilines(&json_value.encoded());
        self.fmt_span("json", &json);
    }

    fn fmt_space(&mut self, space: &Whitespace) {
        let Whitespace { value, .. } = space;
        if !value.is_empty() {
            self.buffer.push_str(value);
        };
    }

    fn fmt_lt(&mut self, lt: &LineTerminator) {
        self.fmt_space(&lt.space0);
        if let Some(v) = &lt.comment {
            self.fmt_comment(v);
        }
        self.buffer.push_str(lt.newline.value.as_str());
    }

    fn fmt_comment(&mut self, comment: &Comment) {
        let comment = format!("#{}", escape_xml(&comment.value));
        self.fmt_span("comment", &comment);
    }

    fn fmt_file(&mut self, file: &File) {
        self.buffer.push_str("file,");
        self.fmt_space(&file.space0);
        self.fmt_filename(&file.filename);
        self.fmt_space(&file.space1);
        self.buffer.push(';');
    }

    fn fmt_base64(&mut self, base64: &Base64) {
        self.buffer.push_str("base64,");
        self.fmt_space(&base64.space0);
        self.fmt_span("base64", &base64.encoded);
        self.fmt_space(&base64.space1);
        self.buffer.push(';');
    }

    fn fmt_hex(&mut self, hex: &Hex) {
        self.buffer.push_str("hex,");
        self.fmt_space(&hex.space0);
        self.fmt_span("hex", &hex.encoded);
        self.fmt_space(&hex.space1);
        self.buffer.push(';');
    }

    fn fmt_regex(&mut self, regex: &Regex) {
        let s = str::replace(regex.inner.as_str(), "/", "\\/");
        let regex = format!("/{s}/");
        self.fmt_span("regex", &regex);
    }

    fn fmt_template(&mut self, template: &Template) {
        let s = template.to_encoded_string();
        self.fmt_string(&escape_xml(&s));
    }

    fn fmt_expr(&mut self, expr: &Expr) {
        let expr = format!("{{{{{}}}}}", &expr.to_string());
        self.fmt_span("expr", &expr);
    }

    fn fmt_filter(&mut self, filter: &Filter) {
        self.fmt_filter_value(&filter.value);
    }

    fn fmt_filter_value(&mut self, filter_value: &FilterValue) {
        match filter_value {
            FilterValue::Count => self.fmt_span("filter-type", "count"),
            FilterValue::DaysAfterNow => self.fmt_span("filter-type", "daysAfterNow"),
            FilterValue::DaysBeforeNow => self.fmt_span("filter-type", "daysBeforeNow"),
            FilterValue::Decode { space0, encoding } => {
                self.fmt_span("filter-type", "decode");
                self.fmt_space(space0);
                self.fmt_template(encoding);
            }
            FilterValue::Format { space0, fmt } => {
                self.fmt_span("filter-type", "format");
                self.fmt_space(space0);
                self.fmt_template(fmt);
            }
            FilterValue::HtmlEscape => self.fmt_span("filter-type", "htmlEscape"),
            FilterValue::HtmlUnescape => self.fmt_span("filter-type", "htmlUnescape"),
            FilterValue::Nth { space0, n: value } => {
                self.fmt_span("filter-type", "nth");
                self.fmt_space(space0);
                self.fmt_number(value);
            }
            FilterValue::Regex { space0, value } => {
                self.fmt_span("filter-type", "regex");
                self.fmt_space(space0);
                self.fmt_regex_value(value);
            }
            FilterValue::Replace {
                space0,
                old_value,
                space1,
                new_value,
            } => {
                self.fmt_span("filter-type", "replace");
                self.fmt_space(space0);
                self.fmt_regex_value(old_value);
                self.fmt_space(space1);
                self.fmt_template(new_value);
            }
            FilterValue::Split { space0, sep } => {
                self.fmt_span("filter-type", "split");
                self.fmt_space(space0);
                self.fmt_template(sep);
            }
            FilterValue::ToDate { space0, fmt } => {
                self.fmt_span("filter-type", "toDate");
                self.fmt_space(space0);
                self.fmt_template(fmt);
            }
            FilterValue::ToInt => self.fmt_span("filter-type", "toInt"),
            FilterValue::UrlDecode => self.fmt_span("filter-type", "urlDecode"),
            FilterValue::UrlEncode => self.fmt_span("filter-type", "urlEncode"),
            FilterValue::XPath { space0, expr } => {
                self.fmt_span("filter-type", "xpath");
                self.fmt_space(space0);
                self.fmt_template(expr);
            }
        };
    }

    fn fmt_lts(&mut self, line_terminators: &[LineTerminator]) {
        for line_terminator in line_terminators {
            self.fmt_span_open("line");
            if line_terminator.newline.value.is_empty() {
                self.buffer.push_str("<br>");
            }
            self.fmt_span_close();
            self.fmt_lt(line_terminator);
        }
    }

    fn fmt_retry(&mut self, retry: Retry) {
        match retry {
            Retry::Finite(n) => self.fmt_span("number", &n.to_string()),
            Retry::Infinite => self.fmt_span("number", "-1"),
            _ => {}
        };
    }
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

impl Template {
    fn to_encoded_string(&self) -> String {
        let mut s = "".to_string();
        if let Some(d) = self.delimiter {
            s.push(d);
        }
        for element in self.elements.iter() {
            let elem_str = match element {
                TemplateElement::String { encoded, .. } => encoded.to_string(),
                TemplateElement::Expression(expr) => format!("{{{{{expr}}}}}"),
            };
            s.push_str(elem_str.as_str())
        }
        if let Some(d) = self.delimiter {
            s.push(d);
        }
        s
    }
}

fn encode_html(s: String) -> String {
    s.replace('>', "&gt;").replace('<', "&lt;")
}

fn format_multilines(s: &str) -> String {
    regex::Regex::new(r"\n|\r\n")
        .unwrap()
        .split(s)
        .map(|l| format!("<span class=\"line\">{}</span>", escape_xml(l)))
        .collect::<Vec<String>>()
        .join("\n")
}

fn pop_str(string: &mut String, suffix: &str) {
    let len = string.len();
    let n = suffix.len();
    let len = len - n;
    string.truncate(len);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiline_string() {
        // ``````
        let multiline_string = MultilineString::OneLineText(Template {
            delimiter: None,
            elements: vec![TemplateElement::String {
                value: "".to_string(),
                encoded: "unused".to_string(),
            }],
            source_info: SourceInfo::new(0, 0, 0, 0),
        });
        let mut fmt = HtmlFormatter::new();
        fmt.fmt_multiline_string(&multiline_string, false);
        assert_eq!(fmt.buffer, "<span class=\"multiline\">``````</span>");
        let mut fmt = HtmlFormatter::new();
        fmt.fmt_multiline_string(&multiline_string, true);
        assert_eq!(
            fmt.buffer,
            "<span class=\"multiline\">\
                <span class=\"line\">``````</span>\
            </span>"
        );

        // ```hello```
        let multiline_string = MultilineString::OneLineText(Template {
            delimiter: None,
            elements: vec![TemplateElement::String {
                value: "hello".to_string(),
                encoded: "unused".to_string(),
            }],
            source_info: SourceInfo::new(0, 0, 0, 0),
        });
        let mut fmt = HtmlFormatter::new();
        fmt.fmt_multiline_string(&multiline_string, false);
        assert_eq!(fmt.buffer, "<span class=\"multiline\">```hello```</span>");
        let mut fmt = HtmlFormatter::new();
        fmt.fmt_multiline_string(&multiline_string, true);
        assert_eq!(
            fmt.buffer,
            "<span class=\"multiline\">\
                <span class=\"line\">```hello```</span>\
            </span>"
        );
        // ```
        // line1
        // line2
        // ```
        let multiline_string = MultilineString::Text(Text {
            space: Whitespace {
                value: "".to_string(),
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
            value: Template {
                delimiter: None,
                elements: vec![TemplateElement::String {
                    value: "line1\nline2\n".to_string(),
                    encoded: "unused".to_string(),
                }],
                source_info: SourceInfo::new(0, 0, 0, 0),
            },
        });
        let mut fmt = HtmlFormatter::new();
        fmt.fmt_multiline_string(&multiline_string, true);
        assert_eq!(
            fmt.buffer,
            "<span class=\"multiline\">\
                <span class=\"line\">```</span>\n\
                <span class=\"line\">line1</span>\n\
                <span class=\"line\">line2</span>\n\
                <span class=\"line\">```</span>\
            </span>"
        );

        let mut fmt = HtmlFormatter::new();
        fmt.fmt_multiline_string(&multiline_string, false);
        assert_eq!(
            fmt.buffer,
            "<span class=\"multiline\">```</span>\
        </span>\n\
        <span class=\"multiline\">\
            <span class=\"line\">line1</span>\n\
            <span class=\"line\">line2</span>\n\
            <span class=\"line\">```</span>"
        );
    }

    #[test]
    fn test_multilines() {
        assert_eq!(
            format_multilines("{\n   \"id\": 1\n}"),
            "<span class=\"line\">{</span>\n\
            <span class=\"line\">   \"id\": 1</span>\n\
            <span class=\"line\">}</span>"
        );
        assert_eq!(
            format_multilines(
                "<?xml version=\"1.0\"?>\n\
            <drink>café</drink>"
            ),
            "<span class=\"line\">&lt;?xml version=\"1.0\"?&gt;</span>\n\
            <span class=\"line\">&lt;drink&gt;café&lt;/drink&gt;</span>"
        );

        assert_eq!(
            format_multilines("Hello\n"),
            "<span class=\"line\">Hello</span>\n\
            <span class=\"line\"></span>"
        );
    }

    #[test]
    fn test_json() {
        let mut fmt = HtmlFormatter::new();
        let value = JsonValue::Object {
            space0: "".to_string(),
            elements: vec![JsonObjectElement {
                space0: "\n   ".to_string(),
                name: Template {
                    delimiter: Some('"'),
                    elements: vec![TemplateElement::String {
                        value: "id".to_string(),
                        encoded: "id".to_string(),
                    }],
                    source_info: SourceInfo::new(0, 0, 0, 0),
                },
                space1: "".to_string(),
                space2: " ".to_string(),
                value: JsonValue::Number("1".to_string()),
                space3: "\n".to_string(),
            }],
        };
        fmt.fmt_json_value(&value);
        assert_eq!(
            fmt.buffer,
            "<span class=\"json\">\
                <span class=\"line\">{</span>\n\
                <span class=\"line\">   \"id\": 1</span>\n\
                <span class=\"line\">}</span>\
            </span>"
        );
    }

    #[test]
    fn test_json_encoded_newline() {
        let mut fmt = HtmlFormatter::new();
        let value = JsonValue::String(Template {
            delimiter: Some('"'),
            elements: vec![TemplateElement::String {
                value: "\n".to_string(),
                encoded: "\\n".to_string(),
            }],
            source_info: SourceInfo::new(0, 0, 0, 0),
        });
        fmt.fmt_json_value(&value);
        assert_eq!(
            fmt.buffer,
            "<span class=\"json\"><span class=\"line\">\"\\n\"</span></span>"
        )
    }

    #[test]
    fn test_xml() {
        let mut fmt = HtmlFormatter::new();
        let value = "<?xml version=\"1.0\"?>\n<drink>café</drink>";
        fmt.fmt_xml(value);
        assert_eq!(
            fmt.buffer,
            "<span class=\"xml\"><span class=\"line\">&lt;?xml version=\"1.0\"?&gt;</span>\n<span class=\"line\">&lt;drink&gt;café&lt;/drink&gt;</span></span>"
        )
    }

    #[test]
    fn test_xml_escape() {
        assert_eq!(escape_xml("hello"), "hello");
        assert_eq!(
            escape_xml("<?xml version=\"1.0\"?>"),
            "&lt;?xml version=\"1.0\"?&gt;"
        );
    }
}
