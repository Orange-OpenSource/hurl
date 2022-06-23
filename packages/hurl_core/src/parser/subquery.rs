/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2022 Orange
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

use super::combinators::*;
use super::primitives::*;
use super::reader::Reader;
use super::ParseResult;
use crate::parser::query::regex_value;
use crate::parser::{Error, ParseError};

pub fn subquery(reader: &mut Reader) -> ParseResult<'static, Subquery> {
    let start = reader.state.pos.clone();
    let value = subquery_value(reader)?;
    let end = reader.state.pos.clone();
    Ok(Subquery {
        source_info: SourceInfo { start, end },
        value,
    })
}

fn subquery_value(reader: &mut Reader) -> ParseResult<'static, SubqueryValue> {
    choice(vec![regex_subquery, count_subquery], reader)
}

fn regex_subquery(reader: &mut Reader) -> ParseResult<'static, SubqueryValue> {
    try_literal("regex", reader)?;
    let space0 = one_or_more_spaces(reader)?;
    let value = regex_value(reader)?;
    Ok(SubqueryValue::Regex { space0, value })
}

fn count_subquery(reader: &mut Reader) -> ParseResult<'static, SubqueryValue> {
    // temporary backward compatibility for countEquals predicate
    if reader.remaining().starts_with("countEquals") {
        return Err(Error {
            pos: reader.state.pos.clone(),
            recoverable: true,
            inner: ParseError::Unexpected {
                character: "countEquals".to_string(),
            },
        });
    }

    try_literal("count", reader)?;
    Ok(SubqueryValue::Count {})
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex_subquery() {
        let mut reader = Reader::init("regex \"Hello (.*)!\"");
        assert_eq!(
            subquery(&mut reader).unwrap(),
            Subquery {
                source_info: SourceInfo::init(1, 1, 1, 20),
                value: SubqueryValue::Regex {
                    space0: Whitespace {
                        value: " ".to_string(),
                        source_info: SourceInfo::init(1, 6, 1, 7)
                    },
                    value: RegexValue::Template(Template {
                        quotes: true,
                        elements: vec![TemplateElement::String {
                            value: "Hello (.*)!".to_string(),
                            encoded: "Hello (.*)!".to_string()
                        }],
                        source_info: SourceInfo::init(1, 7, 1, 20)
                    })
                }
            }
        );
    }

    #[test]
    fn test_count_subquery() {
        let mut reader = Reader::init("count");
        assert_eq!(
            subquery(&mut reader).unwrap(),
            Subquery {
                source_info: SourceInfo::init(1, 1, 1, 6),
                value: SubqueryValue::Count {}
            }
        );
    }
}
