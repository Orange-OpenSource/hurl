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

use crate::jsonpath2::ast::function::{ValueTypeArgument, ValueTypeFunction};

impl ValueTypeArgument {
    pub fn eval(&self, _value: &serde_json::Value) -> Option<serde_json::Value> {
        match self {
            ValueTypeArgument::FilterQuery(_filter_query) => {
                todo!()
            }
            ValueTypeArgument::Function(_value_type_function) => todo!(),
            ValueTypeArgument::Literal(value) => Some(value.eval()),
        }
    }
}

impl ValueTypeFunction {
    #[allow(dead_code)]
    pub fn eval(&self, value: &serde_json::Value) -> Option<serde_json::Value> {
        match self {
            ValueTypeFunction::Length(argument) => {
                let argument = argument.eval(value)?;
                eprintln!(">> length argument: {}", argument);
                length(&argument).map(|n| {
                    let number = serde_json::Number::from_i128(n as i128).unwrap();
                    serde_json::Value::Number(number)
                })
            }
            ValueTypeFunction::Count(_) => {
                // Implement the logic for CountFunction or return None for now
                None
            }
            ValueTypeFunction::Value(_argument) => {
                // Implement the logic for ValueFunction or return None for now
                None
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

// impl MatchFunction {
//     #[allow(dead_code)]
//     pub fn eval(value: &str, pattern: &str) -> bool {
//         todo!() //value.matches(pattern).collect()::<Vec<&str>>().len() > 0
//     }

// }

mod tests {
    #[allow(unused_imports)]
    use super::{ValueTypeArgument, ValueTypeFunction};
    #[allow(unused_imports)]
    use crate::jsonpath2::ast::literal::Literal;
    #[allow(unused_imports)]
    use crate::jsonpath2::ast::segment::Segment;
    #[allow(unused_imports)]
    use crate::jsonpath2::ast::selector::Selector;
    #[allow(unused_imports)]
    use serde_json::json;

    #[test]
    fn test_length() {
        // length(@.authors)
        // length("abc")

        assert_eq!(
            ValueTypeFunction::Length(ValueTypeArgument::Literal(Literal::String(
                "abc".to_string()
            )))
            .eval(&json!({}))
            .unwrap(),
            json!(3)
        );

        // assert_eq!(
        //     ValueTypeFunction::LengthFunction(ValueTypeArgument::FilterQuery(FilterQuery::Rel(
        //         RelQuery::new(vec![Segment::Child(ChildSegment::new(vec![
        //             Selector::Name(NameSelector::new("authors".to_string()))
        //         ]))])
        //     )))
        //     .eval(&json!({"authors": ["a", "b", "c"]}))
        //     .unwrap(),
        //     json!(3)
        // );
    }
}
