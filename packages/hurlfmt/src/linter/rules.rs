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
use hurl_core::ast::{
    Assert, Base64, Body, Bytes, Capture, Comment, Cookie, CookieAttribute, CookieAttributeName,
    CookiePath, DurationOption, Entry, EntryOption, File, FileParam, Filter, FilterValue, GraphQl,
    Hex, HurlFile, KeyValue, LineTerminator, MultilineString, MultilineStringAttribute,
    MultilineStringKind, MultipartParam, OptionKind, Predicate, PredicateFunc, PredicateFuncValue,
    PredicateValue, Query, QueryValue, RegexValue, Request, Response, Section, SectionValue,
    SourceInfo, Template, VariableDefinition, Whitespace,
};
use hurl_core::reader::Pos;
use hurl_core::typing::{Duration, DurationUnit};

/// Returns a new linted instance from this `hurl_file`.
pub fn lint_hurl_file(hurl_file: &HurlFile) -> HurlFile {
    HurlFile {
        entries: hurl_file.entries.iter().map(lint_entry).collect(),
        line_terminators: hurl_file.line_terminators.clone(),
    }
}

fn lint_entry(entry: &Entry) -> Entry {
    let request = lint_request(&entry.request);
    let response = entry.response.as_ref().map(lint_response);
    Entry { request, response }
}

fn lint_request(request: &Request) -> Request {
    let line_terminators = request.line_terminators.clone();
    let space0 = empty_whitespace();
    let method = request.method.clone();
    let space1 = one_whitespace();

    let url = request.url.clone();
    let line_terminator0 = lint_line_terminator(&request.line_terminator0);
    let headers = request.headers.iter().map(lint_key_value).collect();
    let body = request.body.as_ref().map(lint_body);
    let mut sections: Vec<Section> = request.sections.iter().map(lint_section).collect();
    sections.sort_by_key(|k| section_value_index(k.value.clone()));

    let source_info = SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0));
    Request {
        line_terminators,
        space0,
        method,
        space1,
        url,
        line_terminator0,
        headers,
        sections,
        body,
        source_info,
    }
}

fn lint_response(response: &Response) -> Response {
    let line_terminators = response.line_terminators.clone();
    let space0 = empty_whitespace();
    let version = response.version.clone();
    let space1 = response.space1.clone();
    let status = response.status.clone();
    let line_terminator0 = response.line_terminator0.clone();
    let headers = response.headers.iter().map(lint_key_value).collect();
    let mut sections: Vec<Section> = response.sections.iter().map(lint_section).collect();
    sections.sort_by_key(|k| section_value_index(k.value.clone()));
    let body = response.body.clone();

    Response {
        line_terminators,
        space0,
        version,
        space1,
        status,
        line_terminator0,
        headers,
        sections,
        body,
        source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
    }
}

fn lint_section(section: &Section) -> Section {
    let line_terminators = section.line_terminators.clone();
    let line_terminator0 = section.line_terminator0.clone();
    let value = lint_section_value(&section.value);
    Section {
        line_terminators,
        space0: empty_whitespace(),
        value,
        line_terminator0,
        source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
    }
}

fn lint_section_value(section_value: &SectionValue) -> SectionValue {
    match section_value {
        SectionValue::QueryParams(params, short) => {
            SectionValue::QueryParams(params.iter().map(lint_key_value).collect(), *short)
        }
        SectionValue::BasicAuth(param) => {
            SectionValue::BasicAuth(param.as_ref().map(lint_key_value))
        }
        SectionValue::Captures(captures) => {
            SectionValue::Captures(captures.iter().map(lint_capture).collect())
        }
        SectionValue::Asserts(asserts) => {
            SectionValue::Asserts(asserts.iter().map(lint_assert).collect())
        }
        SectionValue::FormParams(params, short) => {
            SectionValue::FormParams(params.iter().map(lint_key_value).collect(), *short)
        }
        SectionValue::MultipartFormData(params, short) => SectionValue::MultipartFormData(
            params.iter().map(lint_multipart_param).collect(),
            *short,
        ),
        SectionValue::Cookies(cookies) => {
            SectionValue::Cookies(cookies.iter().map(lint_cookie).collect())
        }
        SectionValue::Options(options) => {
            SectionValue::Options(options.iter().map(lint_entry_option).collect())
        }
    }
}

fn section_value_index(section_value: SectionValue) -> u32 {
    match section_value {
        // Request sections
        SectionValue::Options(_) => 0,
        SectionValue::QueryParams(_, _) => 1,
        SectionValue::BasicAuth(_) => 2,
        SectionValue::FormParams(_, _) => 3,
        SectionValue::MultipartFormData(_, _) => 4,
        SectionValue::Cookies(_) => 5,
        // Response sections
        SectionValue::Captures(_) => 0,
        SectionValue::Asserts(_) => 1,
    }
}

fn lint_assert(assert: &Assert) -> Assert {
    let filters = assert
        .filters
        .iter()
        .map(|(_, f)| (one_whitespace(), lint_filter(f)))
        .collect();
    Assert {
        line_terminators: assert.line_terminators.clone(),
        space0: empty_whitespace(),
        query: lint_query(&assert.query),
        filters,
        space1: one_whitespace(),
        predicate: lint_predicate(&assert.predicate),
        line_terminator0: assert.line_terminator0.clone(),
    }
}

fn lint_capture(capture: &Capture) -> Capture {
    let filters = capture
        .filters
        .iter()
        .map(|(_, f)| (one_whitespace(), lint_filter(f)))
        .collect();
    let space3 = if capture.redact {
        one_whitespace()
    } else {
        capture.space3.clone()
    };
    Capture {
        line_terminators: capture.line_terminators.clone(),
        space0: empty_whitespace(),
        name: capture.name.clone(),
        space1: empty_whitespace(),
        space2: one_whitespace(),
        query: lint_query(&capture.query),
        filters,
        space3,
        redact: capture.redact,
        line_terminator0: lint_line_terminator(&capture.line_terminator0),
    }
}

fn lint_query(query: &Query) -> Query {
    Query {
        source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
        value: lint_query_value(&query.value),
    }
}

fn lint_query_value(query_value: &QueryValue) -> QueryValue {
    match query_value {
        QueryValue::Status => QueryValue::Status,
        QueryValue::Version => QueryValue::Version,
        QueryValue::Url => QueryValue::Url,
        QueryValue::Header { name, .. } => QueryValue::Header {
            name: name.clone(),
            space0: one_whitespace(),
        },
        QueryValue::Cookie {
            expr: CookiePath { name, attribute },
            ..
        } => {
            let attribute = attribute.as_ref().map(lint_cookie_attribute);
            QueryValue::Cookie {
                space0: one_whitespace(),
                expr: CookiePath {
                    name: name.clone(),
                    attribute,
                },
            }
        }
        QueryValue::Body => QueryValue::Body,
        QueryValue::Xpath { expr, .. } => QueryValue::Xpath {
            expr: expr.clone(),
            space0: one_whitespace(),
        },
        QueryValue::Jsonpath { expr, .. } => QueryValue::Jsonpath {
            expr: expr.clone(),
            space0: one_whitespace(),
        },
        QueryValue::Regex { value, .. } => QueryValue::Regex {
            value: lint_regex_value(value),
            space0: one_whitespace(),
        },
        QueryValue::Variable { name, .. } => QueryValue::Variable {
            name: name.clone(),
            space0: one_whitespace(),
        },
        QueryValue::Duration => QueryValue::Duration,
        QueryValue::Bytes => QueryValue::Bytes,
        QueryValue::Sha256 => QueryValue::Sha256,
        QueryValue::Md5 => QueryValue::Md5,
        QueryValue::Certificate {
            attribute_name: field,
            ..
        } => QueryValue::Certificate {
            attribute_name: *field,
            space0: one_whitespace(),
        },
        QueryValue::Ip => QueryValue::Ip,
        QueryValue::Redirects => QueryValue::Redirects,
    }
}

fn lint_regex_value(regex_value: &RegexValue) -> RegexValue {
    match regex_value {
        RegexValue::Template(template) => RegexValue::Template(lint_template(template)),
        RegexValue::Regex(regex) => RegexValue::Regex(regex.clone()),
    }
}

fn lint_cookie_attribute(cookie_attribute: &CookieAttribute) -> CookieAttribute {
    let space0 = empty_whitespace();
    let name = lint_cookie_attribute_name(&cookie_attribute.name);
    let space1 = empty_whitespace();
    CookieAttribute {
        space0,
        name,
        space1,
    }
}

fn lint_cookie_attribute_name(cookie_attribute_name: &CookieAttributeName) -> CookieAttributeName {
    match cookie_attribute_name {
        CookieAttributeName::Value(_) => CookieAttributeName::Value("Value".to_string()),
        CookieAttributeName::Expires(_) => CookieAttributeName::Expires("Expires".to_string()),
        CookieAttributeName::MaxAge(_) => CookieAttributeName::MaxAge("Max-Age".to_string()),
        CookieAttributeName::Domain(_) => CookieAttributeName::Domain("Domain".to_string()),
        CookieAttributeName::Path(_) => CookieAttributeName::Path("Path".to_string()),
        CookieAttributeName::Secure(_) => CookieAttributeName::Secure("Secure".to_string()),
        CookieAttributeName::HttpOnly(_) => CookieAttributeName::HttpOnly("HttpOnly".to_string()),
        CookieAttributeName::SameSite(_) => CookieAttributeName::SameSite("SameSite".to_string()),
    }
}

fn lint_predicate(predicate: &Predicate) -> Predicate {
    Predicate {
        not: predicate.not,
        space0: if predicate.not {
            one_whitespace()
        } else {
            empty_whitespace()
        },
        predicate_func: lint_predicate_func(&predicate.predicate_func),
    }
}

fn lint_predicate_func(predicate_func: &PredicateFunc) -> PredicateFunc {
    PredicateFunc {
        source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
        value: lint_predicate_func_value(&predicate_func.value),
    }
}

fn lint_predicate_func_value(predicate_func_value: &PredicateFuncValue) -> PredicateFuncValue {
    match predicate_func_value {
        PredicateFuncValue::Equal { value, .. } => PredicateFuncValue::Equal {
            space0: one_whitespace(),
            value: lint_predicate_value(value),
        },
        PredicateFuncValue::NotEqual { value, .. } => PredicateFuncValue::NotEqual {
            space0: one_whitespace(),
            value: lint_predicate_value(value),
        },
        PredicateFuncValue::GreaterThan { value, .. } => PredicateFuncValue::GreaterThan {
            space0: one_whitespace(),
            value: lint_predicate_value(value),
        },
        PredicateFuncValue::GreaterThanOrEqual { value, .. } => {
            PredicateFuncValue::GreaterThanOrEqual {
                space0: one_whitespace(),
                value: lint_predicate_value(value),
            }
        }
        PredicateFuncValue::LessThan { value, .. } => PredicateFuncValue::LessThan {
            space0: one_whitespace(),
            value: lint_predicate_value(value),
        },
        PredicateFuncValue::LessThanOrEqual { value, .. } => PredicateFuncValue::LessThanOrEqual {
            space0: one_whitespace(),
            value: lint_predicate_value(value),
        },
        PredicateFuncValue::Contain { value, .. } => PredicateFuncValue::Contain {
            space0: one_whitespace(),
            value: lint_predicate_value(value),
        },

        PredicateFuncValue::Include { value, .. } => PredicateFuncValue::Include {
            space0: one_whitespace(),
            value: lint_predicate_value(value),
        },

        PredicateFuncValue::Match { value, .. } => PredicateFuncValue::Match {
            space0: one_whitespace(),
            value: lint_predicate_value(value),
        },
        PredicateFuncValue::StartWith { value, .. } => PredicateFuncValue::StartWith {
            space0: one_whitespace(),
            value: lint_predicate_value(value),
        },
        PredicateFuncValue::EndWith { value, .. } => PredicateFuncValue::EndWith {
            space0: one_whitespace(),
            value: lint_predicate_value(value),
        },
        PredicateFuncValue::IsInteger => PredicateFuncValue::IsInteger,
        PredicateFuncValue::IsFloat => PredicateFuncValue::IsFloat,
        PredicateFuncValue::IsBoolean => PredicateFuncValue::IsBoolean,
        PredicateFuncValue::IsString => PredicateFuncValue::IsString,
        PredicateFuncValue::IsCollection => PredicateFuncValue::IsCollection,
        PredicateFuncValue::IsDate => PredicateFuncValue::IsDate,
        PredicateFuncValue::IsIsoDate => PredicateFuncValue::IsIsoDate,
        PredicateFuncValue::Exist => PredicateFuncValue::Exist,
        PredicateFuncValue::IsEmpty => PredicateFuncValue::IsEmpty,
        PredicateFuncValue::IsNumber => PredicateFuncValue::IsNumber,
        PredicateFuncValue::IsIpv4 => PredicateFuncValue::IsIpv4,
        PredicateFuncValue::IsIpv6 => PredicateFuncValue::IsIpv6,
    }
}

fn lint_predicate_value(predicate_value: &PredicateValue) -> PredicateValue {
    match predicate_value {
        PredicateValue::String(value) => PredicateValue::String(lint_template(value)),
        PredicateValue::MultilineString(value) => {
            PredicateValue::MultilineString(lint_multiline_string(value))
        }
        PredicateValue::Bool(value) => PredicateValue::Bool(*value),
        PredicateValue::Null => PredicateValue::Null,
        PredicateValue::Number(value) => PredicateValue::Number(value.clone()),
        PredicateValue::File(value) => PredicateValue::File(lint_file(value)),
        PredicateValue::Hex(value) => PredicateValue::Hex(lint_hex(value)),
        PredicateValue::Base64(value) => PredicateValue::Base64(lint_base64(value)),
        PredicateValue::Placeholder(value) => PredicateValue::Placeholder(value.clone()),
        PredicateValue::Regex(value) => PredicateValue::Regex(value.clone()),
    }
}

fn lint_multiline_string(multiline_string: &MultilineString) -> MultilineString {
    let space = empty_whitespace();
    let newline = multiline_string.newline.clone();
    match multiline_string {
        MultilineString {
            attributes,
            kind: MultilineStringKind::Text(value),
            ..
        } => MultilineString {
            attributes: lint_multiline_string_attributes(attributes),
            space,
            newline,
            kind: MultilineStringKind::Text(lint_template(value)),
        },
        MultilineString {
            attributes,
            kind: MultilineStringKind::Json(value),
            ..
        } => MultilineString {
            attributes: lint_multiline_string_attributes(attributes),
            space,
            newline,
            kind: MultilineStringKind::Json(lint_template(value)),
        },
        MultilineString {
            attributes,
            kind: MultilineStringKind::Xml(value),
            ..
        } => MultilineString {
            attributes: lint_multiline_string_attributes(attributes),
            space,
            newline,
            kind: MultilineStringKind::Xml(lint_template(value)),
        },
        MultilineString {
            attributes,
            kind: MultilineStringKind::GraphQl(value),
            ..
        } => MultilineString {
            attributes: lint_multiline_string_attributes(attributes),
            space,
            newline,
            kind: MultilineStringKind::GraphQl(lint_graphql(value)),
        },
    }
}

fn lint_multiline_string_attributes(
    attributes: &[MultilineStringAttribute],
) -> Vec<MultilineStringAttribute> {
    attributes.to_vec()
}

fn lint_graphql(graphql: &GraphQl) -> GraphQl {
    let value = lint_template(&graphql.value);
    let variables = graphql.variables.clone();
    GraphQl { value, variables }
}

fn lint_cookie(cookie: &Cookie) -> Cookie {
    cookie.clone()
}

fn lint_body(body: &Body) -> Body {
    let line_terminators = body.line_terminators.clone();
    let space0 = empty_whitespace();
    let value = lint_bytes(&body.value);
    let line_terminator0 = body.line_terminator0.clone();
    Body {
        line_terminators,
        space0,
        value,
        line_terminator0,
    }
}

fn lint_bytes(bytes: &Bytes) -> Bytes {
    match bytes {
        Bytes::File(value) => Bytes::File(lint_file(value)),
        Bytes::Base64(value) => Bytes::Base64(lint_base64(value)),
        Bytes::Hex(value) => Bytes::Hex(lint_hex(value)),
        Bytes::Json(value) => Bytes::Json(value.clone()),
        Bytes::OnelineString(value) => Bytes::OnelineString(lint_template(value)),
        Bytes::MultilineString(value) => Bytes::MultilineString(lint_multiline_string(value)),
        Bytes::Xml(value) => Bytes::Xml(value.clone()),
    }
}

fn lint_base64(base64: &Base64) -> Base64 {
    Base64 {
        space0: empty_whitespace(),
        value: base64.value.clone(),
        source: base64.source.clone(),
        space1: empty_whitespace(),
    }
}

fn lint_hex(hex: &Hex) -> Hex {
    Hex {
        space0: empty_whitespace(),
        value: hex.value.clone(),
        source: hex.source.clone(),
        space1: empty_whitespace(),
    }
}

fn lint_file(file: &File) -> File {
    File {
        space0: empty_whitespace(),
        filename: lint_template(&file.filename),
        space1: empty_whitespace(),
    }
}

fn lint_key_value(key_value: &KeyValue) -> KeyValue {
    KeyValue {
        line_terminators: key_value.line_terminators.clone(),
        space0: empty_whitespace(),
        key: key_value.key.clone(),
        space1: empty_whitespace(),
        space2: if key_value.value.elements.is_empty() {
            empty_whitespace()
        } else {
            one_whitespace()
        },
        value: key_value.value.clone(),
        line_terminator0: key_value.line_terminator0.clone(),
    }
}

fn lint_multipart_param(multipart_param: &MultipartParam) -> MultipartParam {
    match multipart_param {
        MultipartParam::Param(param) => MultipartParam::Param(lint_key_value(param)),
        MultipartParam::FileParam(file_param) => {
            MultipartParam::FileParam(lint_file_param(file_param))
        }
    }
}

fn lint_file_param(file_param: &FileParam) -> FileParam {
    let line_terminators = file_param.line_terminators.clone();
    let space0 = file_param.space0.clone();
    let key = file_param.key.clone();
    let space1 = file_param.space1.clone();
    let space2 = file_param.space2.clone();
    let value = file_param.value.clone();
    let line_terminator0 = file_param.line_terminator0.clone();
    FileParam {
        line_terminators,
        space0,
        key,
        space1,
        space2,
        value,
        line_terminator0,
    }
}

fn empty_whitespace() -> Whitespace {
    Whitespace {
        value: String::new(),
        source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
    }
}

fn one_whitespace() -> Whitespace {
    Whitespace {
        value: " ".to_string(),
        source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
    }
}

fn lint_line_terminator(line_terminator: &LineTerminator) -> LineTerminator {
    let space0 = match line_terminator.comment {
        None => empty_whitespace(),
        Some(_) => Whitespace {
            value: line_terminator.space0.value.clone(),
            source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
        },
    };
    let comment = line_terminator.comment.as_ref().map(lint_comment);
    let newline = Whitespace {
        value: if line_terminator.newline.value.is_empty() {
            String::new()
        } else {
            "\n".to_string()
        },
        source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
    };
    LineTerminator {
        space0,
        comment,
        newline,
    }
}

fn lint_comment(comment: &Comment) -> Comment {
    Comment {
        value: if comment.value.starts_with(' ') {
            comment.value.clone()
        } else {
            format!(" {}", comment.value)
        },
        source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
    }
}

fn lint_template(template: &Template) -> Template {
    template.clone()
}

fn lint_entry_option(entry_option: &EntryOption) -> EntryOption {
    EntryOption {
        line_terminators: entry_option.line_terminators.clone(),
        space0: empty_whitespace(),
        space1: empty_whitespace(),
        space2: one_whitespace(),
        kind: lint_option_kind(&entry_option.kind),
        line_terminator0: entry_option.line_terminator0.clone(),
    }
}

fn lint_option_kind(option_kind: &OptionKind) -> OptionKind {
    match option_kind {
        OptionKind::Delay(duration) => {
            OptionKind::Delay(lint_duration_option(duration, DurationUnit::MilliSecond))
        }
        OptionKind::RetryInterval(duration) => {
            OptionKind::RetryInterval(lint_duration_option(duration, DurationUnit::MilliSecond))
        }
        OptionKind::Variable(var_def) => OptionKind::Variable(lint_variable_definition(var_def)),
        _ => option_kind.clone(),
    }
}

fn lint_duration_option(
    duration_option: &DurationOption,
    default_unit: DurationUnit,
) -> DurationOption {
    match duration_option {
        DurationOption::Literal(duration) => {
            DurationOption::Literal(lint_duration(duration, default_unit))
        }
        DurationOption::Placeholder(expr) => DurationOption::Placeholder(expr.clone()),
    }
}

fn lint_duration(duration: &Duration, default_unit: DurationUnit) -> Duration {
    let value = duration.value.clone();
    let unit = Some(duration.unit.unwrap_or(default_unit));
    Duration { value, unit }
}

fn lint_filter(filter: &Filter) -> Filter {
    Filter {
        source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
        value: lint_filter_value(&filter.value),
    }
}

fn lint_filter_value(filter_value: &FilterValue) -> FilterValue {
    match filter_value {
        FilterValue::Regex { value, .. } => FilterValue::Regex {
            space0: one_whitespace(),
            value: lint_regex_value(value),
        },
        f => f.clone(),
    }
}

fn lint_variable_definition(var_def: &VariableDefinition) -> VariableDefinition {
    VariableDefinition {
        space0: empty_whitespace(),
        space1: empty_whitespace(),
        ..var_def.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hurl_file() {
        let hurl_file = HurlFile {
            entries: vec![],
            line_terminators: vec![],
        };
        let hurl_file_linted = HurlFile {
            entries: vec![],
            line_terminators: vec![],
        };
        assert_eq!(lint_hurl_file(&hurl_file), hurl_file_linted);
    }

    #[test]
    fn test_entry() {
        let entry = HurlFile {
            entries: vec![],
            line_terminators: vec![],
        };
        let entry_linted = HurlFile {
            entries: vec![],
            line_terminators: vec![],
        };
        assert_eq!(lint_hurl_file(&entry), entry_linted);
    }
}
