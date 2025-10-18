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
use std::str::FromStr;

use hurl_core::ast::{SourceInfo, Template};

use crate::http::{Url, UrlError};
use crate::runner::template::eval_template;
use crate::runner::{RunnerError, RunnerErrorKind, Value, VariableSet};

/// Returns the value of a query parameter `param` in a URL `value`.
pub fn eval_url_query_param(
    value: &Value,
    param: &Template,
    variables: &VariableSet,
    source_info: SourceInfo,
    assert: bool,
) -> Result<Option<Value>, RunnerError> {
    let param = eval_template(param, variables)?;

    match value {
        Value::String(url) => match Url::from_str(url) {
            Ok(url) => {
                let params = url.query_params();
                let result = params.iter().find(|p| p.name == param);
                match result {
                    Some(param) => Ok(Some(Value::String(param.value.clone()))),
                    None => Ok(None),
                }
            }
            Err(UrlError { url, reason }) => {
                let kind = RunnerErrorKind::InvalidUrl {
                    url: url.to_string(),
                    message: reason,
                };
                Err(RunnerError::new(source_info, kind, assert))
            }
        },
        v => {
            let kind = RunnerErrorKind::FilterInvalidInput(v.kind().to_string());
            Err(RunnerError::new(source_info, kind, assert))
        }
    }
}

#[cfg(test)]
mod tests {
    use hurl_core::ast::{Filter, FilterValue, SourceInfo, Template, TemplateElement, Whitespace};
    use hurl_core::reader::Pos;
    use hurl_core::types::ToSource;

    use super::*;
    use crate::runner::filter::eval::eval_filter;
    use crate::runner::{Value, VariableSet};

    // Helper function to return a new filter given a `param`
    fn new_param_filter(param: &str) -> Filter {
        // urlQueryParam "text"
        Filter {
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
            value: FilterValue::UrlQueryParam {
                space0: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(14, 0), Pos::new(15, 0)),
                },
                param: Template::new(
                    None,
                    vec![TemplateElement::String {
                        value: param.to_string(),
                        source: param.to_source(),
                    }],
                    SourceInfo::new(Pos::new(15, 0), Pos::new(15 + param.len(), 0)),
                ),
            },
        }
    }

    #[test]
    fn eval_filter_url_query_param_ok() {
        let variables = VariableSet::new();

        let filter = new_param_filter("text");

        let url = "http://localhost:8000/api/v1/doc?text=你好世界";

        let ret = eval_filter(&filter, &Value::String(url.to_string()), &variables, false);

        assert_eq!(ret.unwrap().unwrap(), Value::String("你好世界".to_string()));
    }

    #[test]
    fn eval_filter_url_query_param_ok_not_found() {
        let variables = VariableSet::new();

        let filter = new_param_filter("length");

        let url = "http://localhost:8000/api/v1/doc?text=你好世界";

        let ret = eval_filter(&filter, &Value::String(url.to_string()), &variables, false);

        assert_eq!(ret.unwrap(), None);
    }

    #[test]
    fn eval_filter_url_query_param_ko_invalid_input() {
        let variables = VariableSet::new();

        let filter = new_param_filter("text");

        let url = vec![0xc4, 0xe3, 0xba, 0xc3, 0xca, 0xc0, 0xbd, 0xe7];

        let ret = eval_filter(&filter, &Value::Bytes(url), &variables, false);

        assert_eq!(
            ret.unwrap_err().kind,
            RunnerErrorKind::FilterInvalidInput("bytes".to_string())
        );
    }
    #[test]
    fn eval_filter_url_query_param_ko_invalid_url() {
        let variables = VariableSet::new();

        let filter = new_param_filter("text");

        let url = "localhost:8000/api/v1/doc?text=你好世界";

        let ret = eval_filter(&filter, &Value::String(url.to_string()), &variables, false);

        assert_eq!(
            ret.unwrap_err().kind,
            RunnerErrorKind::InvalidUrl {
                url: "localhost:8000/api/v1/doc?text=你好世界".to_string(),
                message: "Missing scheme <http://> or <https://>".to_string()
            }
        );
    }
}
