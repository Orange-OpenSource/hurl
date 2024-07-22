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

use hurl_core::ast::{SourceInfo, Template};

use crate::runner::template::eval_template;
use crate::runner::xpath::{Document, Format, XPathError};
use crate::runner::{RunnerError, RunnerErrorKind, Value};

pub fn eval_xpath(
    value: &Value,
    expr: &Template,
    variables: &HashMap<String, Value>,
    source_info: SourceInfo,
    assert: bool,
) -> Result<Option<Value>, RunnerError> {
    match value {
        Value::String(xml) => {
            // The filter will use the HTML parser that should also work with XML input
            let Ok(doc) = Document::parse(xml, Format::Html) else {
                return Err(RunnerError::new(
                    source_info,
                    RunnerErrorKind::QueryInvalidXml,
                    false,
                ));
            };
            eval_xpath_doc(&doc, expr, variables)
        }
        v => {
            let kind = RunnerErrorKind::FilterInvalidInput(v._type());
            Err(RunnerError::new(source_info, kind, assert))
        }
    }
}

pub fn eval_xpath_doc(
    doc: &Document,
    expr: &Template,
    variables: &HashMap<String, Value>,
) -> Result<Option<Value>, RunnerError> {
    let expr_str = eval_template(expr, variables)?;
    let result = doc.eval_xpath(&expr_str);
    match result {
        Ok(value) => Ok(Some(value)),
        Err(XPathError::Eval) => Err(RunnerError::new(
            expr.source_info,
            RunnerErrorKind::QueryInvalidXpathEval,
            false,
        )),
        Err(XPathError::Unsupported) => {
            panic!("Unsupported xpath {expr}"); // good usecase for panic - I could not reproduce this usecase myself
        }
    }
}
