/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2023 Orange
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

use hurl_core::ast::{SourceInfo, Template};

use crate::runner::template::eval_template;
use crate::runner::{xpath, Error, RunnerError, Value};

pub fn eval_xpath(
    value: &Value,
    expr: &Template,
    variables: &HashMap<String, Value>,
    source_info: &SourceInfo,
    assert: bool,
) -> Result<Option<Value>, Error> {
    match value {
        Value::String(xml) => {
            // The filter will use the HTML parser that should also work with XML input
            let is_html = true;
            eval_xpath_string(xml, expr, variables, source_info, is_html)
        }
        v => Err(Error {
            source_info: source_info.clone(),
            inner: RunnerError::FilterInvalidInput(v._type()),
            assert,
        }),
    }
}

pub fn eval_xpath_string(
    xml: &str,
    expr_template: &Template,
    variables: &HashMap<String, Value>,
    source_info: &SourceInfo,
    is_html: bool,
) -> Result<Option<Value>, Error> {
    let expr = eval_template(expr_template, variables)?;
    let result = if is_html {
        xpath::eval_html(xml, &expr)
    } else {
        xpath::eval_xml(xml, &expr)
    };
    match result {
        Ok(value) => Ok(Some(value)),
        Err(xpath::XpathError::InvalidXml) => Err(Error {
            source_info: source_info.clone(),
            inner: RunnerError::QueryInvalidXml,
            assert: false,
        }),
        Err(xpath::XpathError::InvalidHtml) => Err(Error {
            source_info: source_info.clone(),
            inner: RunnerError::QueryInvalidXml,
            assert: false,
        }),
        Err(xpath::XpathError::Eval) => Err(Error {
            source_info: expr_template.source_info.clone(),
            inner: RunnerError::QueryInvalidXpathEval,
            assert: false,
        }),
        Err(xpath::XpathError::Unsupported) => {
            panic!("Unsupported xpath {expr}"); // good usecase for panic - I could not reproduce this usecase myself
        }
    }
}

#[cfg(test)]
pub mod tests {}
