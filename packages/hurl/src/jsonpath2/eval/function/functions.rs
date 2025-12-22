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

use regex::Regex;
use serde_json::Value;

use crate::jsonpath2::ast::function::functions::{LogicalTypeFunction, ValueTypeFunction};
use crate::jsonpath2::ast::function::LogicalType;

impl ValueTypeFunction {
    #[allow(dead_code)]
    pub fn eval(
        &self,
        current_value: &serde_json::Value,
        root_value: &serde_json::Value,
    ) -> Option<serde_json::Value> {
        match self {
            ValueTypeFunction::Length(argument) => {
                let argument = argument.eval(current_value, root_value)?;
                length(&argument).map(|n| {
                    let number = serde_json::Number::from_i128(n as i128).unwrap();
                    serde_json::Value::Number(number)
                })
            }
            ValueTypeFunction::Count(argument) => {
                let nodelist = argument.eval(current_value, root_value);
                Some(serde_json::Value::Number(serde_json::Number::from(
                    nodelist.len(),
                )))
            }
            ValueTypeFunction::Value(argument) => {
                let nodelist = argument.eval(current_value, root_value);
                if nodelist.len() == 1 {
                    Some(nodelist[0].clone())
                } else {
                    None
                }
            }
        }
    }
}

impl LogicalTypeFunction {
    #[allow(dead_code)]
    pub fn eval(
        &self,
        current_value: &serde_json::Value,
        root_value: &serde_json::Value,
    ) -> LogicalType {
        match self {
            LogicalTypeFunction::Match(string_argument, pattern_argument) => {
                let string = string_argument.eval(current_value, root_value);
                let pattern = pattern_argument.eval(current_value, root_value);

                let s = if let Some(Value::String(value)) = string {
                    value
                } else {
                    return false;
                };
                let p = if let Some(Value::String(value)) = pattern {
                    value
                } else {
                    return false;
                };

                // use total match
                let p = format!("^{}$", p);
                if let Ok(regex) = Regex::new(&p) {
                    regex.is_match(&s)
                } else {
                    false
                }
            }
            LogicalTypeFunction::Search(string_argument, pattern_argument) => {
                let string = string_argument.eval(current_value, root_value);
                let pattern = pattern_argument.eval(current_value, root_value);

                let s = if let Some(Value::String(value)) = string {
                    value
                } else {
                    return false;
                };
                let p = if let Some(Value::String(value)) = pattern {
                    value
                } else {
                    return false;
                };

                if let Ok(regex) = Regex::new(&p) {
                    regex.is_match(&s)
                } else {
                    false
                }
            }
        }
    }
}

#[allow(dead_code)]
pub fn length(value: &serde_json::Value) -> Option<usize> {
    match value {
        serde_json::Value::Array(arr) => Some(arr.len()),
        serde_json::Value::Object(map) => Some(map.len()),
        serde_json::Value::String(s) => Some(s.chars().count()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::jsonpath2::ast::function::argument::{NodesTypeArgument, ValueTypeArgument};
    use crate::jsonpath2::ast::literal::{Literal, Number};
    use crate::jsonpath2::ast::query::{Query, RelativeQuery};
    use crate::jsonpath2::ast::segment::{ChildSegment, DescendantSegment, Segment};
    use crate::jsonpath2::ast::selector::{NameSelector, Selector, WildcardSelector};
    use crate::jsonpath2::ast::singular_query::{
        RelativeSingularQuery, SingularQuery, SingularQuerySegment,
    };
    use serde_json::json;

    #[test]
    fn test_length() {
        // length("abc")
        let length_function = ValueTypeFunction::Length(ValueTypeArgument::Literal(
            Literal::String("abc".to_string()),
        ));
        assert_eq!(
            length_function.eval(&json!({}), &json!({})).unwrap(),
            json!(3)
        );

        // length(1)
        let length_function = ValueTypeFunction::Length(ValueTypeArgument::Literal(
            Literal::Number(Number::Integer(1)),
        ));
        assert!(length_function.eval(&json!({}), &json!({})).is_none());

        // length(@.authors)
        let length_function = ValueTypeFunction::Length(ValueTypeArgument::SingularQuery(
            SingularQuery::Relative(RelativeSingularQuery::new(vec![
                SingularQuerySegment::Name(NameSelector::new("authors".to_string())),
            ])),
        ));
        assert_eq!(
            length_function
                .eval(&json!({"authors": ["a", "b", "c"]}), &json!({}))
                .unwrap(),
            json!(3)
        );
        assert_eq!(
            length_function
                .eval(
                    &json!({"authors": {"a":"a", "b": "b", "c": "c"}}),
                    &json!({})
                )
                .unwrap(),
            json!(3)
        );
        assert!(length_function.eval(&json!({}), &json!({})).is_none());
    }

    #[test]
    fn test_count() {
        // count(@.*.author)
        let count_function = ValueTypeFunction::Count(NodesTypeArgument::FilterQuery(
            Query::RelativeQuery(RelativeQuery::new(vec![
                Segment::Child(ChildSegment::new(vec![Selector::Wildcard(
                    WildcardSelector,
                )])),
                Segment::Child(ChildSegment::new(vec![Selector::Name(NameSelector::new(
                    "author".to_string(),
                ))])),
            ])),
        ));
        assert_eq!(
            count_function
                .eval(&json!([{"author": "Bob"},{"author": "Bill"}]), &json!({}))
                .unwrap(),
            json!(2)
        );
    }

    #[test]
    fn test_match() {
        // match(@.date, "1974-05-..")
        let string_argument = ValueTypeArgument::SingularQuery(SingularQuery::Relative(
            RelativeSingularQuery::new(vec![SingularQuerySegment::Name(NameSelector::new(
                "date".to_string(),
            ))]),
        ));
        let pattern_argument =
            ValueTypeArgument::Literal(Literal::String("1974-05-..".to_string()));
        let match_function = LogicalTypeFunction::Match(string_argument, pattern_argument);

        assert!(match_function.eval(&json!({"date": "1974-05-01"}), &json!({})));
        assert!(!match_function.eval(&json!({"date": "74-05-01"}), &json!({})));
        assert!(!match_function.eval(&json!({"date": "1974-04-01"}), &json!({})));
        assert!(!match_function.eval(&json!({"_date": "1974-05-01"}), &json!({})));

        // match with regex . (1 character)
        let match_function = LogicalTypeFunction::Match(
            ValueTypeArgument::SingularQuery(SingularQuery::Relative(RelativeSingularQuery::new(
                vec![],
            ))),
            ValueTypeArgument::Literal(Literal::String(".".to_string())),
        );
        assert!(match_function.eval(&json!("a"), &json!({})));
        assert!(match_function.eval(&json!("\r"), &json!({}))); // JavaScript is the exception, not the rule
        assert!(match_function.eval(&json!("\u{2028}"), &json!({}))); // line separator
        assert!(match_function.eval(&json!("\u{2029}"), &json!({}))); // paragraph separator
        assert!(!match_function.eval(&json!(""), &json!({})));
        assert!(!match_function.eval(&json!("\n"), &json!({})));
        assert!(!match_function.eval(&json!("ab"), &json!({})));

        // match with regex [.b] dot or b (not any character)
        let match_function = LogicalTypeFunction::Match(
            ValueTypeArgument::SingularQuery(SingularQuery::Relative(RelativeSingularQuery::new(
                vec![],
            ))),
            ValueTypeArgument::Literal(Literal::String("a[.b]c".to_string())),
        );
        assert!(match_function.eval(&json!("a.c"), &json!({})));
        assert!(match_function.eval(&json!("abc"), &json!({})));
        assert!(!match_function.eval(&json!("axc"), &json!({})));
    }

    #[test]
    fn test_search() {
        // search(@.author, "[BR]ob")
        let string_argument = ValueTypeArgument::SingularQuery(SingularQuery::Relative(
            RelativeSingularQuery::new(vec![SingularQuerySegment::Name(NameSelector::new(
                "author".to_string(),
            ))]),
        ));
        let pattern_argument = ValueTypeArgument::Literal(Literal::String("[BR]ob".to_string()));
        let search_function = LogicalTypeFunction::Search(string_argument, pattern_argument);

        assert!(search_function.eval(&json!({"author": "Bob Dylan"}), &json!({})));
        assert!(search_function.eval(&json!({"author": "Robert De Niro"}), &json!({})));
        assert!(!search_function.eval(&json!({"author": "Christiano Ronaldo"}), &json!({})));
    }

    #[test]
    fn test_value() {
        // value(@..color)
        let value_function =
            ValueTypeFunction::Value(NodesTypeArgument::FilterQuery(Query::RelativeQuery(
                RelativeQuery::new(vec![Segment::Descendant(DescendantSegment::new(vec![
                    Selector::Name(NameSelector::new("color".to_string())),
                ]))]),
            )));
        assert_eq!(
            value_function
                .eval(&json!({"color": "red"}), &json!({}))
                .unwrap(),
            json!("red")
        );
        assert!(value_function.eval(&json!({}), &json!({})).is_none());
    }
}
