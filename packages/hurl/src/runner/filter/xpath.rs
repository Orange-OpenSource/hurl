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
use hurl_core::ast::{SourceInfo, Template};

use crate::runner::template::eval_template;
use crate::runner::xpath::{Document, Format, XPathError};
use crate::runner::{RunnerError, RunnerErrorKind, Value, VariableSet};

/// Evaluates a XPath expression `expr` against a `value`.
pub fn eval_xpath(
    value: &Value,
    expr: &Template,
    variables: &VariableSet,
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
            let kind = RunnerErrorKind::FilterInvalidInput(v.kind().to_string());
            Err(RunnerError::new(source_info, kind, assert))
        }
    }
}

pub fn eval_xpath_doc(
    doc: &Document,
    expr: &Template,
    variables: &VariableSet,
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

#[cfg(test)]
mod tests {
    use hurl_core::ast::{Filter, FilterValue, SourceInfo, Template, TemplateElement, Whitespace};
    use hurl_core::reader::Pos;
    use hurl_core::typing::ToSource;

    use super::*;
    use crate::runner::filter::eval::eval_filter;
    use crate::runner::VariableSet;

    /// Helper function to return a new filter given a `expr`
    fn new_xpath_filter(expr: &str) -> Filter {
        // Example: xpath "string(//body/text())"
        Filter {
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
            value: FilterValue::XPath {
                space0: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(6, 1), Pos::new(7, 1)),
                },
                expr: Template {
                    delimiter: None,
                    elements: vec![TemplateElement::String {
                        value: expr.to_string(),
                        source: expr.to_source(),
                    }],
                    source_info: SourceInfo::new(Pos::new(7, 1), Pos::new(7 + expr.len(), 1)),
                },
            },
        }
    }

    #[test]
    fn eval_filter_xpath_doc_ok() {
        let variables = VariableSet::new();

        let html = "<html><body>你好世界</body></html>";
        let filter = new_xpath_filter("string(//body/text())");
        let ret = eval_filter(&filter, &Value::String(html.to_string()), &variables, false);

        assert_eq!(ret.unwrap().unwrap(), Value::String("你好世界".to_string()));
    }

    #[test]
    fn eval_filter_xpath_doc_ko_invalid_xpath() {
        let variables = VariableSet::new();

        let html = "<html><body>你好世界</body></html>";
        let filter = new_xpath_filter("str(//body/text())");
        let ret = eval_filter(&filter, &Value::String(html.to_string()), &variables, false);

        assert_eq!(
            ret.unwrap_err().kind,
            RunnerErrorKind::QueryInvalidXpathEval
        );
    }

    #[test]
    fn eval_filter_xpath_doc_ko_invalid_xml() {
        let variables = VariableSet::new();

        let html = "";
        let filter = new_xpath_filter("string(//body/text())");
        let ret = eval_filter(&filter, &Value::String(html.to_string()), &variables, false);

        assert_eq!(ret.unwrap_err().kind, RunnerErrorKind::QueryInvalidXml);
    }

    #[test]
    fn eval_filter_xpath_doc_ko_invalid_input() {
        let variables = VariableSet::new();

        let filter = new_xpath_filter("string(//body/text())");
        let ret = eval_filter(
            &filter,
            &Value::Bytes(vec![0xc4, 0xe3, 0xba, 0xc3, 0xca, 0xc0, 0xbd, 0xe7]),
            &variables,
            false,
        );

        assert_eq!(
            ret.unwrap_err().kind,
            RunnerErrorKind::FilterInvalidInput("bytes".to_string())
        );
    }
}
