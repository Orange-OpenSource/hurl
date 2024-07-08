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
use std::collections::HashMap;

use hurl_core::ast::{RegexValue, SourceInfo};

use crate::runner::regex::eval_regex_value;
use crate::runner::{RunnerError, RunnerErrorKind, Value};

pub fn eval_regex(
    value: &Value,
    regex_value: &RegexValue,
    variables: &HashMap<String, Value>,
    source_info: SourceInfo,
    assert: bool,
) -> Result<Option<Value>, RunnerError> {
    let re = eval_regex_value(regex_value, variables)?;
    match value {
        Value::String(s) => match re.captures(s.as_str()) {
            Some(captures) => match captures.get(1) {
                Some(v) => Ok(Some(Value::String(v.as_str().to_string()))),
                None => Ok(None),
            },
            None => Ok(None),
        },
        v => {
            let kind = RunnerErrorKind::FilterInvalidInput(v._type());
            Err(RunnerError::new(source_info, kind, assert))
        }
    }
}

#[cfg(test)]
pub mod tests {
    use std::collections::HashMap;

    use hurl_core::ast::{
        Filter, FilterValue, RegexValue, SourceInfo, Template, TemplateElement, Whitespace,
    };
    use hurl_core::reader::Pos;

    use crate::runner::filter::eval::eval_filter;
    use crate::runner::{RunnerErrorKind, Value};

    #[test]
    fn eval_filter_regex() {
        // regex "Hello (.*)!"
        let variables = HashMap::new();
        let whitespace = Whitespace {
            value: String::new(),
            source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
        };
        let filter = Filter {
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 20)),
            value: FilterValue::Regex {
                space0: whitespace,
                value: RegexValue::Template(Template {
                    delimiter: None,
                    elements: vec![TemplateElement::String {
                        value: "Hello (.*)!".to_string(),
                        encoded: "Hello (.*)!".to_string(),
                    }],
                    source_info: SourceInfo::new(Pos::new(1, 7), Pos::new(1, 20)),
                }),
            },
        };
        assert_eq!(
            eval_filter(
                &filter,
                &Value::String("Hello Bob!".to_string()),
                &variables,
                false,
            )
            .unwrap()
            .unwrap(),
            Value::String("Bob".to_string())
        );

        let error = eval_filter(&filter, &Value::Bool(true), &variables, false)
            .err()
            .unwrap();
        assert_eq!(
            error.source_info,
            SourceInfo::new(Pos::new(1, 1), Pos::new(1, 20))
        );
        assert_eq!(
            error.kind,
            RunnerErrorKind::FilterInvalidInput("boolean".to_string())
        );
    }

    #[test]
    fn eval_filter_invalid_regex() {
        let variables = HashMap::new();
        let whitespace = Whitespace {
            value: String::new(),
            source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
        };
        let filter = Filter {
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 20)),
            value: FilterValue::Regex {
                space0: whitespace,
                value: RegexValue::Template(Template {
                    delimiter: None,
                    elements: vec![TemplateElement::String {
                        value: "???".to_string(),
                        encoded: "???".to_string(),
                    }],
                    source_info: SourceInfo::new(Pos::new(1, 7), Pos::new(1, 20)),
                }),
            },
        };
        let error = eval_filter(
            &filter,
            &Value::String("Hello Bob!".to_string()),
            &variables,
            false,
        )
        .err()
        .unwrap();
        assert_eq!(
            error.source_info,
            SourceInfo::new(Pos::new(1, 7), Pos::new(1, 20))
        );
        assert_eq!(error.kind, RunnerErrorKind::InvalidRegex);
    }
}
