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
use crate::ast::{Filter, FilterValue, IntegerValue, SourceInfo, Whitespace};
use crate::combinator::{choice, ParseError as ParseErrorTrait};
use crate::parser::number::integer;
use crate::parser::primitives::{one_or_more_spaces, try_literal, zero_or_more_spaces};
use crate::parser::query::regex_value;
use crate::parser::string::quoted_template;
use crate::parser::{placeholder, ParseError, ParseErrorKind, ParseResult};
use crate::reader::Reader;

pub fn filters(reader: &mut Reader) -> ParseResult<Vec<(Whitespace, Filter)>> {
    let mut filters = vec![];
    loop {
        let save = reader.cursor();
        let space = zero_or_more_spaces(reader)?;
        if space.value.is_empty() {
            break;
        }
        match filter(reader) {
            Ok(f) => {
                filters.push((space, f));
            }
            Err(e) => {
                if e.recoverable {
                    reader.seek(save);
                    break;
                } else {
                    return Err(e);
                }
            }
        }
    }
    Ok(filters)
}

pub fn filter(reader: &mut Reader) -> ParseResult<Filter> {
    let start = reader.cursor();
    let value = choice(
        &[
            base64_decode_filter,
            base64_encode_filter,
            base64_url_safe_decode_filter,
            base64_url_safe_encode_filter,
            count_filter,
            days_after_now_filter,
            days_before_now_filter,
            decode_filter,
            first_filter,
            format_filter,
            date_format_filter,
            html_decode_filter,
            html_encode_filter,
            jsonpath_filter,
            last_filter,
            location_filter,
            nth_filter,
            regex_filter,
            replace_regex_filter,
            replace_filter,
            split_filter,
            to_date_filter,
            to_float_filter,
            to_hex_filter,
            to_int_filter,
            to_string_filter,
            url_decode_filter,
            url_encode_filter,
            url_query_param_filter,
            xpath_filter,
        ],
        reader,
    )
    .map_err(|e| {
        if e.recoverable {
            let kind = ParseErrorKind::Expecting {
                value: "filter".to_string(),
            };
            ParseError::new(e.pos, true, kind)
        } else {
            e
        }
    })?;
    let end = reader.cursor();
    let source_info = SourceInfo {
        start: start.pos,
        end: end.pos,
    };
    Ok(Filter { source_info, value })
}

fn base64_decode_filter(reader: &mut Reader) -> ParseResult<FilterValue> {
    try_literal("base64Decode", reader)?;
    Ok(FilterValue::Base64Decode)
}

fn base64_encode_filter(reader: &mut Reader) -> ParseResult<FilterValue> {
    try_literal("base64Encode", reader)?;
    Ok(FilterValue::Base64Encode)
}

fn base64_url_safe_decode_filter(reader: &mut Reader) -> ParseResult<FilterValue> {
    try_literal("base64UrlSafeDecode", reader)?;
    Ok(FilterValue::Base64UrlSafeDecode)
}

fn base64_url_safe_encode_filter(reader: &mut Reader) -> ParseResult<FilterValue> {
    try_literal("base64UrlSafeEncode", reader)?;
    Ok(FilterValue::Base64UrlSafeEncode)
}

fn count_filter(reader: &mut Reader) -> ParseResult<FilterValue> {
    try_literal("count", reader)?;
    Ok(FilterValue::Count)
}

fn days_after_now_filter(reader: &mut Reader) -> ParseResult<FilterValue> {
    try_literal("daysAfterNow", reader)?;
    Ok(FilterValue::DaysAfterNow)
}

fn days_before_now_filter(reader: &mut Reader) -> ParseResult<FilterValue> {
    try_literal("daysBeforeNow", reader)?;
    Ok(FilterValue::DaysBeforeNow)
}

fn decode_filter(reader: &mut Reader) -> ParseResult<FilterValue> {
    try_literal("decode", reader)?;
    let space0 = one_or_more_spaces(reader)?;
    let encoding = quoted_template(reader)?;
    Ok(FilterValue::Decode { space0, encoding })
}

fn first_filter(reader: &mut Reader) -> ParseResult<FilterValue> {
    try_literal("first", reader)?;
    Ok(FilterValue::First)
}

fn format_filter(reader: &mut Reader) -> ParseResult<FilterValue> {
    try_literal("format", reader)?;
    let space0 = one_or_more_spaces(reader)?;
    let fmt = quoted_template(reader)?;
    Ok(FilterValue::Format { space0, fmt })
}

fn date_format_filter(reader: &mut Reader) -> ParseResult<FilterValue> {
    try_literal("dateFormat", reader)?;
    let space0 = one_or_more_spaces(reader)?;
    let fmt = quoted_template(reader)?;
    Ok(FilterValue::DateFormat { space0, fmt })
}

fn html_encode_filter(reader: &mut Reader) -> ParseResult<FilterValue> {
    try_literal("htmlEscape", reader)?;
    Ok(FilterValue::HtmlEscape)
}

fn html_decode_filter(reader: &mut Reader) -> ParseResult<FilterValue> {
    try_literal("htmlUnescape", reader)?;
    Ok(FilterValue::HtmlUnescape)
}

fn jsonpath_filter(reader: &mut Reader) -> ParseResult<FilterValue> {
    try_literal("jsonpath", reader)?;
    let space0 = one_or_more_spaces(reader)?;
    let expr = quoted_template(reader).map_err(|e| e.to_non_recoverable())?;
    Ok(FilterValue::JsonPath { space0, expr })
}

fn last_filter(reader: &mut Reader) -> ParseResult<FilterValue> {
    try_literal("last", reader)?;
    Ok(FilterValue::Last)
}

fn location_filter(reader: &mut Reader) -> ParseResult<FilterValue> {
    try_literal("location", reader)?;
    Ok(FilterValue::Location)
}

fn nth_filter(reader: &mut Reader) -> ParseResult<FilterValue> {
    try_literal("nth", reader)?;
    let space0 = one_or_more_spaces(reader)?;
    let n = integer_value(reader)?;
    Ok(FilterValue::Nth { space0, n })
}

fn integer_value(reader: &mut Reader) -> ParseResult<IntegerValue> {
    let start = reader.cursor();
    match integer(reader) {
        Ok(v) => Ok(IntegerValue::Literal(v)),
        Err(_) => {
            reader.seek(start);
            let placeholder = placeholder::parse(reader).map_err(|e| {
                let kind = ParseErrorKind::Expecting {
                    value: "integer".to_string(),
                };
                ParseError::new(e.pos, false, kind)
            })?;
            Ok(IntegerValue::Placeholder(placeholder))
        }
    }
}

fn regex_filter(reader: &mut Reader) -> ParseResult<FilterValue> {
    try_literal("regex", reader)?;
    let space0 = one_or_more_spaces(reader)?;
    let value = regex_value(reader)?;
    Ok(FilterValue::Regex { space0, value })
}

fn replace_filter(reader: &mut Reader) -> ParseResult<FilterValue> {
    try_literal("replace", reader)?;
    let space0 = one_or_more_spaces(reader)?;
    let old_value = quoted_template(reader).map_err(|e| e.to_non_recoverable())?;
    let space1 = one_or_more_spaces(reader)?;
    let new_value = quoted_template(reader).map_err(|e| e.to_non_recoverable())?;
    Ok(FilterValue::Replace {
        space0,
        old_value,
        space1,
        new_value,
    })
}

fn replace_regex_filter(reader: &mut Reader) -> ParseResult<FilterValue> {
    try_literal("replaceRegex", reader)?;
    let space0 = one_or_more_spaces(reader)?;
    let pattern = regex_value(reader)?;
    let space1 = one_or_more_spaces(reader)?;
    let new_value = quoted_template(reader).map_err(|e| e.to_non_recoverable())?;
    Ok(FilterValue::ReplaceRegex {
        space0,
        pattern,
        space1,
        new_value,
    })
}

fn split_filter(reader: &mut Reader) -> ParseResult<FilterValue> {
    try_literal("split", reader)?;
    let space0 = one_or_more_spaces(reader)?;
    let sep = quoted_template(reader).map_err(|e| e.to_non_recoverable())?;
    Ok(FilterValue::Split { space0, sep })
}

fn to_date_filter(reader: &mut Reader) -> ParseResult<FilterValue> {
    try_literal("toDate", reader)?;
    let space0 = one_or_more_spaces(reader)?;
    let fmt = quoted_template(reader)?;
    Ok(FilterValue::ToDate { space0, fmt })
}

fn to_float_filter(reader: &mut Reader) -> ParseResult<FilterValue> {
    try_literal("toFloat", reader)?;
    Ok(FilterValue::ToFloat)
}

fn to_hex_filter(reader: &mut Reader) -> ParseResult<FilterValue> {
    try_literal("toHex", reader)?;
    Ok(FilterValue::ToHex)
}

fn to_int_filter(reader: &mut Reader) -> ParseResult<FilterValue> {
    try_literal("toInt", reader)?;
    Ok(FilterValue::ToInt)
}

fn to_string_filter(reader: &mut Reader) -> ParseResult<FilterValue> {
    try_literal("toString", reader)?;
    Ok(FilterValue::ToString)
}

fn url_encode_filter(reader: &mut Reader) -> ParseResult<FilterValue> {
    try_literal("urlEncode", reader)?;
    Ok(FilterValue::UrlEncode)
}

fn url_decode_filter(reader: &mut Reader) -> ParseResult<FilterValue> {
    try_literal("urlDecode", reader)?;
    Ok(FilterValue::UrlDecode)
}

fn url_query_param_filter(reader: &mut Reader) -> ParseResult<FilterValue> {
    try_literal("urlQueryParam", reader)?;
    let space0 = one_or_more_spaces(reader)?;
    let param = quoted_template(reader).map_err(|e| e.to_non_recoverable())?;
    Ok(FilterValue::UrlQueryParam { space0, param })
}

fn xpath_filter(reader: &mut Reader) -> ParseResult<FilterValue> {
    try_literal("xpath", reader)?;
    let space0 = one_or_more_spaces(reader)?;
    let expr = quoted_template(reader).map_err(|e| e.to_non_recoverable())?;
    Ok(FilterValue::XPath { space0, expr })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ParseErrorKind;
    use crate::reader::Pos;

    #[test]
    fn test_count() {
        let mut reader = Reader::new("count");
        assert_eq!(
            filter(&mut reader).unwrap(),
            Filter {
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 6)),
                value: FilterValue::Count,
            }
        );
    }

    #[test]
    fn test_error() {
        let mut reader = Reader::new("xcount");
        let err = filter(&mut reader).err().unwrap();
        assert_eq!(
            err.kind,
            ParseErrorKind::Expecting {
                value: "filter".to_string()
            }
        );
        assert_eq!(err.pos, Pos { line: 1, column: 1 });
        assert!(err.recoverable);

        let mut reader = Reader::new("regex 1");
        let err = filter(&mut reader).err().unwrap();
        assert_eq!(
            err.kind,
            ParseErrorKind::Expecting {
                value: "\" or /".to_string()
            }
        );
        assert_eq!(err.pos, Pos { line: 1, column: 7 });
        assert!(!err.recoverable);
    }
}
