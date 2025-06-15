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
//! Walker traverses an AST in depth-first order. Each overridden visit method has full control over
//! what happens with its node, it can do its own traversal of the node's children, call `visit::walk_*`
//! to apply the default traversal algorithm, or prevent deeper traversal by doing nothing.
//!
//! Code heavily inspired from <https://github.com/rust-lang/rust/blob/master/compiler/rustc_ast/src/visit.rs>
use crate::ast::{
    Assert, Base64, Body, BooleanOption, Bytes, Capture, Comment, Cookie, CookiePath, CountOption,
    DurationOption, Entry, EntryOption, File, FilenameParam, FilenameValue, Filter, FilterValue,
    Hex, HurlFile, JsonValue, KeyValue, LineTerminator, Method, MultilineString, MultipartParam,
    NaturalOption, Number, OptionKind, Placeholder, Predicate, PredicateFuncValue, PredicateValue,
    Query, QueryValue, Regex, RegexValue, Request, Response, Section, SectionValue, StatusValue,
    Template, VariableDefinition, VariableValue, VersionValue, Whitespace, U64,
};
use crate::typing::{Count, Duration, DurationUnit, SourceString, ToSource};

/// Each method of the `Visitor` trait is a hook to be potentially overridden. Each method's default
/// implementation recursively visits the substructure of the input via the corresponding `walk` method;
/// e.g., the `visit_item` method by default calls `visit::walk_item`.
#[allow(unused_variables)]
pub trait Visitor: Sized {
    fn visit_assert(&mut self, assert: &Assert) {
        walk_assert(self, assert);
    }

    fn visit_base64(&mut self, value: &Base64) {
        walk_base64(self, value);
    }

    fn visit_base64_value(&mut self, value: &[u8], source: &SourceString) {}

    fn visit_body(&mut self, body: &Body) {
        walk_body(self, body);
    }

    fn visit_bool(&mut self, value: bool) {}

    fn visit_bool_option(&mut self, option: &BooleanOption) {
        walk_bool_option(self, option);
    }

    fn visit_capture(&mut self, capture: &Capture) {
        walk_capture(self, capture);
    }

    fn visit_cookie(&mut self, cookie: &Cookie) {
        walk_cookie(self, cookie);
    }

    fn visit_cookie_path(&mut self, path: &CookiePath) {}

    fn visit_comment(&mut self, comment: &Comment) {}

    fn visit_count(&mut self, count: Count) {
        walk_count(self, count);
    }

    fn visit_count_option(&mut self, option: &CountOption) {
        walk_count_option(self, option);
    }

    fn visit_duration(&mut self, duration: &Duration) {
        walk_duration(self, duration);
    }

    fn visit_duration_option(&mut self, option: &DurationOption) {
        walk_duration_option(self, option);
    }

    fn visit_duration_unit(&mut self, unit: DurationUnit) {}

    fn visit_entry(&mut self, entry: &Entry) {
        walk_entry(self, entry);
    }

    fn visit_entry_option(&mut self, option: &EntryOption) {
        walk_entry_option(self, option);
    }

    fn visit_file(&mut self, file: &File) {
        walk_file(self, file);
    }

    fn visit_filename_param(&mut self, param: &FilenameParam) {
        walk_filename_param(self, param);
    }

    fn visit_filename_value(&mut self, value: &FilenameValue) {
        walk_filename_value(self, value);
    }

    fn visit_filename(&mut self, filename: &Template) {}

    fn visit_filter(&mut self, filter: &Filter) {
        walk_filter(self, filter);
    }

    fn visit_filter_kind(&mut self, kind: &FilterValue) {}

    fn visit_header(&mut self, header: &KeyValue) {
        walk_header(self, header);
    }

    fn visit_hex(&mut self, hex: &Hex) {
        walk_hex(self, hex);
    }

    fn visit_hex_value(&mut self, value: &[u8], source: &SourceString) {}

    fn visit_hurl_file(&mut self, file: &HurlFile) {
        walk_hurl_file(self, file);
    }

    fn visit_i64(&mut self, n: i64) {}

    fn visit_json_body(&mut self, json: &JsonValue) {}

    fn visit_kv(&mut self, kv: &KeyValue) {
        walk_kv(self, kv);
    }

    fn visit_lt(&mut self, lt: &LineTerminator) {
        walk_lt(self, lt);
    }

    fn visit_literal(&mut self, lit: &'static str) {}

    fn visit_method(&mut self, method: &Method) {}

    fn visit_multiline_string(&mut self, string: &MultilineString) {}

    fn visit_natural_option(&mut self, option: &NaturalOption) {
        walk_natural_option(self, option);
    }

    fn visit_not(&mut self, identifier: &'static str) {}

    fn visit_null(&mut self, identifier: &'static str) {}

    fn visit_number(&mut self, number: &Number) {}

    fn visit_placeholder(&mut self, placeholder: &Placeholder) {}

    fn visit_predicate(&mut self, predicate: &Predicate) {
        walk_predicate(self, predicate);
    }

    fn visit_predicate_kind(&mut self, kind: &PredicateFuncValue) {}

    fn visit_predicate_value(&mut self, value: &PredicateValue) {
        walk_predicate_value(self, value);
    }

    fn visit_query(&mut self, query: &Query) {
        walk_query(self, query);
    }

    fn visit_query_kind(&mut self, kind: &QueryValue) {}

    fn visit_request(&mut self, request: &Request) {
        walk_request(self, request);
    }

    fn visit_response(&mut self, response: &Response) {
        walk_response(self, response);
    }

    fn visit_regex(&mut self, regex: &Regex) {}

    fn visit_section(&mut self, section: &Section) {
        walk_section(self, section);
    }

    fn visit_status(&mut self, value: &StatusValue) {}

    fn visit_string(&mut self, value: &str) {}

    fn visit_section_header(&mut self, name: &str) {}

    fn visit_section_value(&mut self, section_value: &SectionValue) {
        walk_section_value(self, section_value);
    }

    fn visit_template(&mut self, template: &Template) {}

    fn visit_url(&mut self, url: &Template) {}

    fn visit_u64(&mut self, n: &U64) {}

    fn visit_usize(&mut self, n: usize) {}

    fn visit_variable_def(&mut self, def: &VariableDefinition) {
        walk_variable_def(self, def);
    }

    fn visit_variable_name(&mut self, name: &str) {}

    fn visit_variable_value(&mut self, value: &VariableValue) {
        walk_variable_value(self, value);
    }

    fn visit_version(&mut self, value: &VersionValue) {}

    fn visit_xml_body(&mut self, xml: &str) {}

    fn visit_whitespace(&mut self, ws: &Whitespace) {}
}

pub fn walk_assert<V: Visitor>(visitor: &mut V, assert: &Assert) {
    assert.line_terminators.iter().for_each(|lt| {
        visitor.visit_lt(lt);
    });
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

pub fn walk_base64<V: Visitor>(visitor: &mut V, base64: &Base64) {
    visitor.visit_literal("base64,");
    visitor.visit_whitespace(&base64.space0);
    visitor.visit_base64_value(&base64.value, &base64.source);
    visitor.visit_whitespace(&base64.space1);
    visitor.visit_literal(";");
}

pub fn walk_body<V: Visitor>(visitor: &mut V, body: &Body) {
    body.line_terminators.iter().for_each(|lt| {
        visitor.visit_lt(lt);
    });
    visitor.visit_whitespace(&body.space0);
    match &body.value {
        Bytes::Json(value) => visitor.visit_json_body(value),
        Bytes::Xml(value) => visitor.visit_xml_body(value),
        Bytes::MultilineString(value) => visitor.visit_multiline_string(value),
        Bytes::OnelineString(value) => visitor.visit_template(value),
        Bytes::Base64(value) => visitor.visit_base64(value),
        Bytes::File(value) => visitor.visit_file(value),
        Bytes::Hex(value) => visitor.visit_hex(value),
    }
    visitor.visit_lt(&body.line_terminator0);
}

pub fn walk_bool_option<V: Visitor>(visitor: &mut V, option: &BooleanOption) {
    match option {
        BooleanOption::Literal(value) => visitor.visit_bool(*value),
        BooleanOption::Placeholder(value) => visitor.visit_placeholder(value),
    }
}

pub fn walk_capture<V: Visitor>(visitor: &mut V, capture: &Capture) {
    capture.line_terminators.iter().for_each(|lt| {
        visitor.visit_lt(lt);
    });
    visitor.visit_whitespace(&capture.space0);
    visitor.visit_template(&capture.name);
    visitor.visit_whitespace(&capture.space1);
    visitor.visit_literal(":");
    visitor.visit_whitespace(&capture.space2);
    visitor.visit_query(&capture.query);
    for (space, filter) in capture.filters.iter() {
        visitor.visit_whitespace(space);
        visitor.visit_filter(filter);
    }
    if let Some(ws) = &capture.space3 {
        visitor.visit_whitespace(ws);
    }
    if capture.redacted {
        // The next node should have been literal to be more correct
        // we visit a string instead to be comptaible with <= 6.1.1 HTML export
        // visitor.visit_literal("redact");
        visitor.visit_string("redact");
    }
    visitor.visit_lt(&capture.line_terminator0);
}

pub fn walk_cookie<V: Visitor>(visitor: &mut V, cookie: &Cookie) {
    cookie.line_terminators.iter().for_each(|lt| {
        visitor.visit_lt(lt);
    });
    visitor.visit_whitespace(&cookie.space0);
    visitor.visit_template(&cookie.name);
    visitor.visit_whitespace(&cookie.space1);
    visitor.visit_literal(":");
    visitor.visit_whitespace(&cookie.space2);
    visitor.visit_template(&cookie.value);
    visitor.visit_lt(&cookie.line_terminator0);
}

pub fn walk_count<V: Visitor>(visitor: &mut V, count: Count) {
    match count {
        Count::Finite(count) => visitor.visit_usize(count),
        Count::Infinite => visitor.visit_i64(-1),
    }
}

pub fn walk_count_option<V: Visitor>(visitor: &mut V, option: &CountOption) {
    match option {
        CountOption::Literal(value) => visitor.visit_count(*value),
        CountOption::Placeholder(value) => visitor.visit_placeholder(value),
    }
}

pub fn walk_duration<V: Visitor>(visitor: &mut V, duration: &Duration) {
    visitor.visit_u64(&duration.value);
    if let Some(unit) = duration.unit {
        visitor.visit_duration_unit(unit);
    }
}

pub fn walk_duration_option<V: Visitor>(visitor: &mut V, option: &DurationOption) {
    match option {
        DurationOption::Literal(value) => visitor.visit_duration(value),
        DurationOption::Placeholder(value) => visitor.visit_placeholder(value),
    }
}

pub fn walk_entry<V: Visitor>(visitor: &mut V, entry: &Entry) {
    visitor.visit_request(&entry.request);
    if let Some(ref response) = entry.response {
        visitor.visit_response(response);
    }
}

pub fn walk_entry_option<V: Visitor>(visitor: &mut V, option: &EntryOption) {
    option.line_terminators.iter().for_each(|lt| {
        visitor.visit_lt(lt);
    });
    visitor.visit_whitespace(&option.space0);
    visitor.visit_string(option.kind.identifier());
    visitor.visit_whitespace(&option.space1);
    visitor.visit_literal(":");
    visitor.visit_whitespace(&option.space2);
    match &option.kind {
        OptionKind::AwsSigV4(value) => visitor.visit_template(value),
        OptionKind::CaCertificate(filename) => visitor.visit_filename(filename),
        OptionKind::ClientCert(filename) => visitor.visit_filename(filename),
        OptionKind::ClientKey(filename) => visitor.visit_filename(filename),
        OptionKind::Compressed(value) => visitor.visit_bool_option(value),
        OptionKind::ConnectTo(value) => visitor.visit_template(value),
        OptionKind::ConnectTimeout(value) => visitor.visit_duration_option(value),
        OptionKind::Delay(value) => visitor.visit_duration_option(value),
        OptionKind::FollowLocation(value) => visitor.visit_bool_option(value),
        OptionKind::FollowLocationTrusted(value) => visitor.visit_bool_option(value),
        OptionKind::Header(value) => visitor.visit_template(value),
        OptionKind::Http10(value) => visitor.visit_bool_option(value),
        OptionKind::Http11(value) => visitor.visit_bool_option(value),
        OptionKind::Http2(value) => visitor.visit_bool_option(value),
        OptionKind::Http3(value) => visitor.visit_bool_option(value),
        OptionKind::Insecure(value) => visitor.visit_bool_option(value),
        OptionKind::IpV4(value) => visitor.visit_bool_option(value),
        OptionKind::IpV6(value) => visitor.visit_bool_option(value),
        OptionKind::LimitRate(value) => visitor.visit_natural_option(value),
        OptionKind::MaxRedirect(value) => visitor.visit_count_option(value),
        OptionKind::MaxTime(value) => visitor.visit_duration_option(value),
        OptionKind::NetRc(value) => visitor.visit_bool_option(value),
        OptionKind::NetRcFile(filename) => visitor.visit_filename(filename),
        OptionKind::NetRcOptional(value) => visitor.visit_bool_option(value),
        OptionKind::Output(filename) => visitor.visit_filename(filename),
        OptionKind::PathAsIs(value) => visitor.visit_bool_option(value),
        OptionKind::PinnedPublicKey(value) => visitor.visit_template(value),
        OptionKind::Proxy(value) => visitor.visit_template(value),
        OptionKind::Repeat(value) => visitor.visit_count_option(value),
        OptionKind::Resolve(value) => visitor.visit_template(value),
        OptionKind::Retry(value) => visitor.visit_count_option(value),
        OptionKind::RetryInterval(value) => visitor.visit_duration_option(value),
        OptionKind::Skip(value) => visitor.visit_bool_option(value),
        OptionKind::UnixSocket(value) => visitor.visit_filename(value),
        OptionKind::User(value) => visitor.visit_template(value),
        OptionKind::Variable(value) => visitor.visit_variable_def(value),
        OptionKind::Verbose(value) => visitor.visit_bool_option(value),
        OptionKind::VeryVerbose(value) => visitor.visit_bool_option(value),
    };
    visitor.visit_lt(&option.line_terminator0);
}

pub fn walk_file<V: Visitor>(visitor: &mut V, file: &File) {
    visitor.visit_literal("file,");
    visitor.visit_whitespace(&file.space0);
    visitor.visit_filename(&file.filename);
    visitor.visit_whitespace(&file.space1);
    visitor.visit_literal(";");
}

pub fn walk_filter<V: Visitor>(visitor: &mut V, filter: &Filter) {
    visitor.visit_filter_kind(&filter.value);
    match &filter.value {
        FilterValue::Base64Decode => {}
        FilterValue::Base64Encode => {}
        FilterValue::Base64UrlSafeDecode => {}
        FilterValue::Base64UrlSafeEncode => {}
        FilterValue::Count => {}
        FilterValue::DaysAfterNow => {}
        FilterValue::DaysBeforeNow => {}
        FilterValue::Decode { space0, encoding } => {
            visitor.visit_whitespace(space0);
            visitor.visit_template(encoding);
        }
        FilterValue::First => {}
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
        FilterValue::Last => {}
        FilterValue::Location => {}
        FilterValue::Nth { space0, n } => {
            visitor.visit_whitespace(space0);
            visitor.visit_i64(n.as_i64());
        }
        FilterValue::Regex { space0, value } => {
            visitor.visit_whitespace(space0);
            match value {
                RegexValue::Template(value) => visitor.visit_template(value),
                RegexValue::Regex(regex) => visitor.visit_regex(regex),
            }
        }
        FilterValue::Replace {
            space0,
            old_value,
            space1,
            new_value,
        } => {
            visitor.visit_whitespace(space0);
            visitor.visit_template(old_value);
            visitor.visit_whitespace(space1);
            visitor.visit_template(new_value);
        }
        FilterValue::ReplaceRegex {
            space0,
            pattern,
            space1,
            new_value,
        } => {
            visitor.visit_whitespace(space0);
            match pattern {
                RegexValue::Template(value) => visitor.visit_template(value),
                RegexValue::Regex(regex) => visitor.visit_regex(regex),
            }
            visitor.visit_whitespace(space1);
            visitor.visit_template(new_value);
        }
        FilterValue::Split { space0, sep } => {
            visitor.visit_whitespace(space0);
            visitor.visit_template(sep);
        }
        FilterValue::ToDate { space0, fmt } => {
            visitor.visit_whitespace(space0);
            visitor.visit_template(fmt);
        }
        FilterValue::ToFloat => {}
        FilterValue::ToHex => {}
        FilterValue::ToInt => {}
        FilterValue::UrlDecode => {}
        FilterValue::UrlEncode => {}
        FilterValue::XPath { space0, expr } => {
            visitor.visit_whitespace(space0);
            visitor.visit_template(expr);
        }
        FilterValue::ToString => {}
        FilterValue::UrlQueryParam { space0, param } => {
            visitor.visit_whitespace(space0);
            visitor.visit_template(param);
        }
    }
}

pub fn walk_filename_param<V: Visitor>(visitor: &mut V, param: &FilenameParam) {
    param.line_terminators.iter().for_each(|lt| {
        visitor.visit_lt(lt);
    });
    visitor.visit_whitespace(&param.space0);
    visitor.visit_template(&param.key);
    visitor.visit_whitespace(&param.space1);
    visitor.visit_literal(":");
    visitor.visit_whitespace(&param.space2);
    visitor.visit_filename_value(&param.value);
    visitor.visit_lt(&param.line_terminator0);
}

pub fn walk_filename_value<V: Visitor>(visitor: &mut V, value: &FilenameValue) {
    visitor.visit_literal("file,");
    visitor.visit_whitespace(&value.space0);
    visitor.visit_filename(&value.filename);
    visitor.visit_whitespace(&value.space1);
    visitor.visit_literal(";");
    visitor.visit_whitespace(&value.space2);
    if let Some(content_type) = &value.content_type {
        visitor.visit_template(content_type);
    }
}

pub fn walk_header<V: Visitor>(visitor: &mut V, header: &KeyValue) {
    visitor.visit_kv(header);
}

pub fn walk_hex<V: Visitor>(visitor: &mut V, hex: &Hex) {
    visitor.visit_literal("hex,");
    visitor.visit_whitespace(&hex.space0);
    visitor.visit_hex_value(&hex.value, &hex.source);
    visitor.visit_whitespace(&hex.space1);
    visitor.visit_literal(";");
}

pub fn walk_hurl_file<V: Visitor>(visitor: &mut V, file: &HurlFile) {
    file.entries.iter().for_each(|e| visitor.visit_entry(e));
    file.line_terminators.iter().for_each(|lt| {
        visitor.visit_lt(lt);
    });
}

pub fn walk_kv<V: Visitor>(visitor: &mut V, kv: &KeyValue) {
    kv.line_terminators.iter().for_each(|lt| {
        visitor.visit_lt(lt);
    });
    visitor.visit_whitespace(&kv.space0);
    visitor.visit_template(&kv.key);
    visitor.visit_whitespace(&kv.space1);
    visitor.visit_literal(":");
    visitor.visit_whitespace(&kv.space2);
    visitor.visit_template(&kv.value);
    visitor.visit_lt(&kv.line_terminator0);
}

pub fn walk_lt<V: Visitor>(visitor: &mut V, lt: &LineTerminator) {
    visitor.visit_whitespace(&lt.space0);
    if let Some(ref comment) = lt.comment {
        visitor.visit_comment(comment);
    }
    visitor.visit_whitespace(&lt.newline);
}

pub fn walk_natural_option<V: Visitor>(visitor: &mut V, option: &NaturalOption) {
    match option {
        NaturalOption::Literal(value) => visitor.visit_u64(value),
        NaturalOption::Placeholder(value) => visitor.visit_placeholder(value),
    }
}

pub fn walk_query<V: Visitor>(visitor: &mut V, query: &Query) {
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
            match value {
                RegexValue::Template(t) => visitor.visit_template(t),
                RegexValue::Regex(r) => visitor.visit_regex(r),
            }
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
            visitor.visit_string(attribute_name.to_source().as_str());
        }
        QueryValue::Body
        | QueryValue::Status
        | QueryValue::Url
        | QueryValue::Duration
        | QueryValue::Bytes
        | QueryValue::Sha256
        | QueryValue::Md5
        | QueryValue::Version
        | QueryValue::Ip
        | QueryValue::Redirects => {}
    }
}

pub fn walk_predicate<V: Visitor>(visitor: &mut V, pred: &Predicate) {
    if pred.not {
        visitor.visit_not("not");
        visitor.visit_whitespace(&pred.space0);
    }
    let kind = &pred.predicate_func.value;
    visitor.visit_predicate_kind(kind);
    match kind {
        PredicateFuncValue::Equal { space0, value } => {
            visitor.visit_whitespace(space0);
            visitor.visit_predicate_value(value);
        }
        PredicateFuncValue::NotEqual { space0, value } => {
            visitor.visit_whitespace(space0);
            visitor.visit_predicate_value(value);
        }
        PredicateFuncValue::GreaterThan { space0, value } => {
            visitor.visit_whitespace(space0);
            visitor.visit_predicate_value(value);
        }
        PredicateFuncValue::GreaterThanOrEqual { space0, value } => {
            visitor.visit_whitespace(space0);
            visitor.visit_predicate_value(value);
        }
        PredicateFuncValue::LessThan { space0, value } => {
            visitor.visit_whitespace(space0);
            visitor.visit_predicate_value(value);
        }
        PredicateFuncValue::LessThanOrEqual { space0, value } => {
            visitor.visit_whitespace(space0);
            visitor.visit_predicate_value(value);
        }
        PredicateFuncValue::StartWith { space0, value } => {
            visitor.visit_whitespace(space0);
            visitor.visit_predicate_value(value);
        }
        PredicateFuncValue::EndWith { space0, value } => {
            visitor.visit_whitespace(space0);
            visitor.visit_predicate_value(value);
        }
        PredicateFuncValue::Contain { space0, value } => {
            visitor.visit_whitespace(space0);
            visitor.visit_predicate_value(value);
        }
        PredicateFuncValue::Include { space0, value } => {
            visitor.visit_whitespace(space0);
            visitor.visit_predicate_value(value);
        }
        PredicateFuncValue::Match { space0, value } => {
            visitor.visit_whitespace(space0);
            visitor.visit_predicate_value(value);
        }
        PredicateFuncValue::IsInteger
        | PredicateFuncValue::IsFloat
        | PredicateFuncValue::IsBoolean
        | PredicateFuncValue::IsString
        | PredicateFuncValue::IsCollection
        | PredicateFuncValue::IsDate
        | PredicateFuncValue::IsIsoDate
        | PredicateFuncValue::Exist
        | PredicateFuncValue::IsEmpty
        | PredicateFuncValue::IsNumber
        | PredicateFuncValue::IsIpv4
        | PredicateFuncValue::IsIpv6 => {}
    }
}

pub fn walk_predicate_value<V: Visitor>(visitor: &mut V, pred_value: &PredicateValue) {
    match pred_value {
        PredicateValue::Base64(value) => visitor.visit_base64(value),
        PredicateValue::Bool(value) => visitor.visit_bool(*value),
        PredicateValue::File(value) => visitor.visit_file(value),
        PredicateValue::Hex(value) => visitor.visit_hex(value),
        PredicateValue::MultilineString(value) => visitor.visit_multiline_string(value),
        PredicateValue::Null => visitor.visit_null("null"),
        PredicateValue::Number(value) => visitor.visit_number(value),
        PredicateValue::Placeholder(placeholder) => visitor.visit_placeholder(placeholder),
        PredicateValue::Regex(value) => visitor.visit_regex(value),
        PredicateValue::String(value) => visitor.visit_template(value),
    }
}

pub fn walk_request<V: Visitor>(visitor: &mut V, request: &Request) {
    request.line_terminators.iter().for_each(|lt| {
        visitor.visit_lt(lt);
    });
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
    if let Some(body) = &request.body {
        visitor.visit_body(body);
    }
}

pub fn walk_response<V: Visitor>(visitor: &mut V, response: &Response) {
    response.line_terminators.iter().for_each(|lt| {
        visitor.visit_lt(lt);
    });
    visitor.visit_whitespace(&response.space0);
    visitor.visit_version(&response.version.value);
    visitor.visit_whitespace(&response.space1);
    visitor.visit_status(&response.status.value);
    visitor.visit_lt(&response.line_terminator0);
    response.headers.iter().for_each(|h| visitor.visit_kv(h));
    response
        .sections
        .iter()
        .for_each(|s| visitor.visit_section(s));
    if let Some(body) = &response.body {
        visitor.visit_body(body);
    }
}

pub fn walk_section<V: Visitor>(visitor: &mut V, section: &Section) {
    section.line_terminators.iter().for_each(|lt| {
        visitor.visit_lt(lt);
    });
    visitor.visit_whitespace(&section.space0);
    let name = format!("[{}]", section.identifier());
    visitor.visit_section_header(&name);
    visitor.visit_lt(&section.line_terminator0);
    visitor.visit_section_value(&section.value);
}

pub fn walk_section_value<V: Visitor>(visitor: &mut V, section_value: &SectionValue) {
    match section_value {
        SectionValue::Asserts(asserts) => asserts.iter().for_each(|a| visitor.visit_assert(a)),
        SectionValue::BasicAuth(Some(auth)) => visitor.visit_kv(auth),
        SectionValue::BasicAuth(_) => {}
        SectionValue::Captures(captures) => captures.iter().for_each(|c| visitor.visit_capture(c)),
        SectionValue::Cookies(cookies) => cookies.iter().for_each(|c| visitor.visit_cookie(c)),
        SectionValue::FormParams(params, _) => params.iter().for_each(|p| visitor.visit_kv(p)),
        SectionValue::MultipartFormData(params, _) => params.iter().for_each(|p| match p {
            MultipartParam::Param(param) => visitor.visit_kv(param),
            MultipartParam::FilenameParam(param) => visitor.visit_filename_param(param),
        }),
        SectionValue::Options(options) => {
            options.iter().for_each(|o| visitor.visit_entry_option(o));
        }
        SectionValue::QueryParams(params, _) => params.iter().for_each(|p| visitor.visit_kv(p)),
    }
}

pub fn walk_variable_def<V: Visitor>(visitor: &mut V, def: &VariableDefinition) {
    visitor.visit_variable_name(&def.name);
    visitor.visit_whitespace(&def.space0);
    visitor.visit_literal("=");
    visitor.visit_whitespace(&def.space1);
    visitor.visit_variable_value(&def.value);
}

pub fn walk_variable_value<V: Visitor>(visitor: &mut V, value: &VariableValue) {
    match value {
        VariableValue::Null => visitor.visit_null("null"),
        VariableValue::Bool(value) => visitor.visit_bool(*value),
        VariableValue::Number(value) => visitor.visit_number(value),
        VariableValue::String(value) => visitor.visit_template(value),
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::visit::Visitor;
    use crate::ast::Assert;
    use crate::parser;

    #[test]
    fn test_walk_assert() {
        struct AssertWalker {
            count: usize,
        }

        impl Visitor for AssertWalker {
            fn visit_assert(&mut self, _assert: &Assert) {
                self.count += 1;
            }
        }

        let mut walker = AssertWalker { count: 0 };
        let content = r#"
GET https://foo.com
HTTP 200
[Asserts]
jsonpath "$.toto[0]" == "tata"
jsonpath "$.toto[1]" == "toto"
jsonpath "$.toto[2]" == "titi"
jsonpath "$.toto[3]" == "tata"
jsonpath "$.toto[4]" == "tutu"

GET https://foo.com
HTTP 200
[Asserts]
status == 200
header "Location" not exists
"#;
        let file = parser::parse_hurl_file(content).unwrap();
        walker.visit_hurl_file(&file);
        assert_eq!(walker.count, 7);
    }
}
