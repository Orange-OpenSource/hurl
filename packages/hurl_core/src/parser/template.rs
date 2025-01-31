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
use crate::ast::{Placeholder, SourceInfo, TemplateElement};
use crate::parser::primitives::zero_or_more_spaces;
use crate::parser::{error, expr, ParseResult};
use crate::reader::{Pos, Reader};
use crate::typing::SourceString;

pub struct EncodedString {
    pub source_info: SourceInfo,
    pub chars: Vec<(char, String, Pos)>,
}

pub fn templatize(encoded_string: EncodedString) -> ParseResult<Vec<TemplateElement>> {
    enum State {
        String,
        Template,
        FirstOpenBracket,
        FirstCloseBracket,
    }

    let mut elements = vec![];

    let mut value = String::new();
    let mut source = SourceString::new();
    let mut state = State::String;
    let mut expression_start = None;

    for (c, s, pos) in encoded_string.chars {
        match state {
            State::String => {
                if s.as_str() == "{" {
                    state = State::FirstOpenBracket;
                } else {
                    value.push(c);
                    source.push_str(&s);
                }
            }

            State::FirstOpenBracket => {
                if s.as_str() == "{" {
                    if !value.is_empty() {
                        elements.push(TemplateElement::String { value, source });
                        value = String::new();
                        source = SourceString::new();
                    }
                    state = State::Template;
                } else {
                    value.push('{');
                    source.push('{');

                    value.push(c);
                    source.push_str(&s);
                    state = State::String;
                }
            }

            State::Template => {
                if expression_start.is_none() {
                    expression_start = Some(pos);
                }
                if s.as_str() == "}" {
                    state = State::FirstCloseBracket;
                } else {
                    value.push(c);
                    source.push_str(&s);
                }
            }

            State::FirstCloseBracket => {
                if s.as_str() == "}" {
                    let mut reader = Reader::with_pos(source.as_str(), expression_start.unwrap());
                    let space0 = zero_or_more_spaces(&mut reader)?;
                    let expr = expr::parse(&mut reader)?;
                    let space1 = zero_or_more_spaces(&mut reader)?;
                    let placeholder = Placeholder {
                        space0,
                        expr,
                        space1,
                    };
                    elements.push(TemplateElement::Placeholder(placeholder));
                    value = String::new();
                    source = SourceString::new();
                    expression_start = None;
                    state = State::String;
                } else {
                    value.push('}');
                    value.push(c);
                    source.push('}');
                    source.push_str(&s);
                }
            }
        }
    }

    match state {
        State::String => {}
        State::FirstOpenBracket => {
            value.push('{');
            source.push('{');
        }
        State::Template | State::FirstCloseBracket => {
            let kind = error::ParseErrorKind::Expecting {
                value: "}}".to_string(),
            };
            return Err(error::ParseError::new(
                encoded_string.source_info.end,
                false,
                kind,
            ));
        }
    }

    if !value.is_empty() {
        elements.push(TemplateElement::String { value, source });
    }
    Ok(elements)
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::ast::{Expr, ExprKind, Placeholder, Variable, Whitespace};
    use crate::typing::ToSource;

    #[test]
    fn test_templatize_empty_string() {
        let encoded_string = EncodedString {
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
            chars: vec![],
        };
        assert_eq!(templatize(encoded_string).unwrap(), vec![]);
    }

    #[test]
    fn test_templatize_hello_world() {
        // Hi\u0020{{name}}!
        let encoded_string = EncodedString {
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 18)),
            chars: vec![
                ('H', "H".to_string(), Pos { line: 1, column: 1 }),
                ('i', "i".to_string(), Pos { line: 1, column: 2 }),
                (' ', "\\u0020".to_string(), Pos { line: 1, column: 3 }),
                ('{', "{".to_string(), Pos { line: 1, column: 9 }),
                (
                    '{',
                    "{".to_string(),
                    Pos {
                        line: 1,
                        column: 10,
                    },
                ),
                (
                    'n',
                    "n".to_string(),
                    Pos {
                        line: 1,
                        column: 11,
                    },
                ),
                (
                    'a',
                    "a".to_string(),
                    Pos {
                        line: 1,
                        column: 12,
                    },
                ),
                (
                    'm',
                    "m".to_string(),
                    Pos {
                        line: 1,
                        column: 13,
                    },
                ),
                (
                    'e',
                    "e".to_string(),
                    Pos {
                        line: 1,
                        column: 14,
                    },
                ),
                (
                    '}',
                    "}".to_string(),
                    Pos {
                        line: 1,
                        column: 15,
                    },
                ),
                (
                    '}',
                    "}".to_string(),
                    Pos {
                        line: 1,
                        column: 16,
                    },
                ),
                (
                    '!',
                    "!".to_string(),
                    Pos {
                        line: 1,
                        column: 17,
                    },
                ),
            ],
        };
        assert_eq!(
            templatize(encoded_string).unwrap(),
            vec![
                TemplateElement::String {
                    value: "Hi ".to_string(),
                    source: "Hi\\u0020".to_source(),
                },
                TemplateElement::Placeholder(Placeholder {
                    space0: Whitespace {
                        value: String::new(),
                        source_info: SourceInfo::new(Pos::new(1, 11), Pos::new(1, 11)),
                    },
                    expr: Expr {
                        kind: ExprKind::Variable(Variable {
                            name: "name".to_string(),
                            source_info: SourceInfo::new(Pos::new(1, 11), Pos::new(1, 15)),
                        }),
                        source_info: SourceInfo::new(Pos::new(1, 11), Pos::new(1, 15)),
                    },
                    space1: Whitespace {
                        value: String::new(),
                        source_info: SourceInfo::new(Pos::new(1, 15), Pos::new(1, 15)),
                    },
                }),
                TemplateElement::String {
                    value: "!".to_string(),
                    source: "!".to_source(),
                },
            ]
        );
    }

    #[test]
    fn test_templatize_expression_only() {
        // {{x}}!
        let encoded_string = EncodedString {
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 7)),
            chars: vec![
                ('{', "{".to_string(), Pos { line: 1, column: 1 }),
                ('{', "{".to_string(), Pos { line: 1, column: 2 }),
                ('x', "x".to_string(), Pos { line: 1, column: 3 }),
                ('}', "}".to_string(), Pos { line: 1, column: 4 }),
                ('}', "}".to_string(), Pos { line: 1, column: 4 }),
            ],
        };
        assert_eq!(
            templatize(encoded_string).unwrap(),
            vec![TemplateElement::Placeholder(Placeholder {
                space0: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 3), Pos::new(1, 3)),
                },
                expr: Expr {
                    kind: ExprKind::Variable(Variable {
                        name: "x".to_string(),
                        source_info: SourceInfo::new(Pos::new(1, 3), Pos::new(1, 4)),
                    }),
                    source_info: SourceInfo::new(Pos::new(1, 3), Pos::new(1, 4))
                },
                space1: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 4), Pos::new(1, 4)),
                },
            }),]
        );
    }

    #[test]
    fn test_templatize_error() {
        // missing closing
        // {{x
        let encoded_string = EncodedString {
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 4)),
            chars: vec![
                ('{', "{".to_string(), Pos { line: 1, column: 1 }),
                ('{', "{".to_string(), Pos { line: 1, column: 2 }),
                ('x', "x".to_string(), Pos { line: 1, column: 3 }),
            ],
        };
        let error = templatize(encoded_string).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 4 });
        assert_eq!(
            error.kind,
            error::ParseErrorKind::Expecting {
                value: "}}".to_string()
            }
        );
        assert!(!error.recoverable);
    }

    #[test]
    fn test_escape_bracket() {
        // \{\{
        // This is a valid string "{{"
        let encoded_string = EncodedString {
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 4)),
            chars: vec![
                ('{', "\\{".to_string(), Pos::new(1, 1)),
                ('{', "\\{".to_string(), Pos::new(1, 2)),
            ],
        };
        assert_eq!(
            templatize(encoded_string).unwrap(),
            vec![TemplateElement::String {
                value: "{{".to_string(),
                source: "\\{\\{".to_source(),
            },]
        );
    }
}
