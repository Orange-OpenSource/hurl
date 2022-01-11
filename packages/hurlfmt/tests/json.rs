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

use std::fs;

use proptest::prelude::prop::test_runner::TestRunner;
use proptest::prelude::*;

use hurl_core::ast::*;
use hurl_core::parser::{parse_json, Reader};
use hurlfmt::format::{Token, Tokenizable};

fn whitespace() -> BoxedStrategy<String> {
    prop_oneof![
        Just("".to_string()),
        Just(" ".to_string()),
        Just("  ".to_string()),
    ]
    .boxed()
}

// region strategy scalar/leaves

fn value_number() -> BoxedStrategy<JsonValue> {
    prop_oneof![
        Just(JsonValue::Number("0".to_string())),
        Just(JsonValue::Number("1".to_string())),
        Just(JsonValue::Number("1.33".to_string())),
        Just(JsonValue::Number("-100".to_string()))
    ]
    .boxed()
}

fn value_boolean() -> BoxedStrategy<JsonValue> {
    prop_oneof![
        Just(JsonValue::Boolean(true)),
        Just(JsonValue::Boolean(false)),
    ]
    .boxed()
}

fn value_string() -> BoxedStrategy<JsonValue> {
    let source_info = SourceInfo::init(0, 0, 0, 0);
    let variable = Variable {
        name: "name".to_string(),
        source_info: source_info.clone(),
    };
    prop_oneof![
        Just(JsonValue::String(Template {
            elements: vec![],
            quotes: true,
            source_info: source_info.clone()
        })),
        Just(JsonValue::String(Template {
            elements: vec![TemplateElement::String {
                encoded: "Hello".to_string(),
                value: "Hello".to_string(),
            }],
            quotes: true,
            source_info: source_info.clone()
        })),
        Just(JsonValue::String(Template {
            elements: vec![
                TemplateElement::String {
                    encoded: "Hello\\u0020 ".to_string(),
                    value: "Hello ".to_string(),
                },
                TemplateElement::Expression(Expr {
                    space0: Whitespace {
                        value: "".to_string(),
                        source_info: source_info.clone()
                    },
                    variable,
                    space1: Whitespace {
                        value: "".to_string(),
                        source_info: source_info.clone()
                    },
                })
            ],
            quotes: true,
            source_info
        })),
    ]
    .boxed()
}

// endregion

// region strategy value

fn value() -> BoxedStrategy<JsonValue> {
    let leaf = prop_oneof![value_boolean(), value_string(), value_number(),];
    leaf.prop_recursive(
        8,   // 8 levels deep
        256, // Shoot for maximum size of 256 nodes
        10,  // We put up to 10 items per collection
        |value| {
            prop_oneof![
                // Lists
                (whitespace()).prop_map(|space0| JsonValue::List {
                    space0,
                    elements: vec![]
                }),
                (whitespace(), whitespace(), value.clone()).prop_map(|(space0, space1, value)| {
                    JsonValue::List {
                        space0,
                        elements: vec![JsonListElement {
                            space0: "".to_string(),
                            value,
                            space1,
                        }],
                    }
                }),
                (
                    whitespace(),
                    whitespace(),
                    value_number(),
                    whitespace(),
                    whitespace(),
                    value_number()
                )
                    .prop_map(
                        |(space00, space01, value0, space10, space11, value1)| JsonValue::List {
                            space0: space00,
                            elements: vec![
                                JsonListElement {
                                    space0: "".to_string(),
                                    value: value0,
                                    space1: space01
                                },
                                JsonListElement {
                                    space0: space10,
                                    value: value1,
                                    space1: space11
                                },
                            ]
                        }
                    ),
                (
                    whitespace(),
                    whitespace(),
                    value_boolean(),
                    whitespace(),
                    whitespace(),
                    value_boolean()
                )
                    .prop_map(
                        |(space00, space01, value0, space10, space11, value1)| JsonValue::List {
                            space0: space00,
                            elements: vec![
                                JsonListElement {
                                    space0: "".to_string(),
                                    value: value0,
                                    space1: space01
                                },
                                JsonListElement {
                                    space0: space10,
                                    value: value1,
                                    space1: space11
                                },
                            ]
                        }
                    ),
                (
                    whitespace(),
                    whitespace(),
                    value_string(),
                    whitespace(),
                    whitespace(),
                    value_string()
                )
                    .prop_map(
                        |(space00, space01, value0, space10, space11, value1)| JsonValue::List {
                            space0: space00,
                            elements: vec![
                                JsonListElement {
                                    space0: "".to_string(),
                                    value: value0,
                                    space1: space01
                                },
                                JsonListElement {
                                    space0: space10,
                                    value: value1,
                                    space1: space11
                                },
                            ]
                        }
                    ),
                // Object
                (whitespace()).prop_map(|space0| JsonValue::Object {
                    space0,
                    elements: vec![]
                }),
                (
                    whitespace(),
                    whitespace(),
                    whitespace(),
                    value,
                    whitespace()
                )
                    .prop_map(|(space0, space1, space2, value, space3)| {
                        JsonValue::Object {
                            space0,
                            elements: vec![JsonObjectElement {
                                space0: "".to_string(),
                                name: Template {
                                    quotes: false,
                                    elements: vec![TemplateElement::String {
                                        value: "key1".to_string(),
                                        encoded: "key1".to_string(),
                                    }],
                                    source_info: SourceInfo::init(1, 1, 1, 1),
                                },
                                space1,
                                space2,
                                value,
                                space3,
                            }],
                        }
                    }),
            ]
        },
    )
    .boxed()
}

// endregion

// region test-echo

fn format_token(token: Token) -> String {
    match token {
        Token::Whitespace(s)
        | Token::Number(s)
        | Token::Boolean(s)
        | Token::String(s)
        | Token::Keyword(s)
        | Token::Quote(s)
        | Token::QueryType(s)
        | Token::CodeVariable(s)
        | Token::CodeDelimiter(s) => s,
        _ => panic!("invalid token {:?}", token),
    }
}

fn format_value(value: JsonValue) -> String {
    let tokens = value.tokenize();
    //eprintln!("{:?}", tokens);
    tokens
        .iter()
        .map(|t| format_token(t.clone()))
        .collect::<Vec<String>>()
        .join("")
}

#[test]
fn test_echo() {
    let mut runner = TestRunner::default();
    //let mut runner = TestRunner::new(ProptestConfig::with_cases(10000));
    runner
        .run(&value(), |value| {
            //eprintln!("value={:#?}", value);
            let s = format_value(value);
            eprintln!("s={}", s);
            let mut reader = Reader::init(s.as_str());
            let parsed_value = parse_json(&mut reader).unwrap();
            assert_eq!(format_value(parsed_value), s);

            Ok(())
        })
        .unwrap();
    //assert_eq!(1,2);
}

#[test]
fn test_parse_files() {
    let paths = fs::read_dir("tests/json").unwrap();

    for p in paths {
        let path = p.unwrap().path();
        println!("parsing json file {}", path.display());
        let s = fs::read_to_string(path).expect("Something went wrong reading the file");
        let mut reader = Reader::init(s.as_str());
        let parsed_value = parse_json(&mut reader).unwrap();

        assert_eq!(format_value(parsed_value), s);
    }
}
