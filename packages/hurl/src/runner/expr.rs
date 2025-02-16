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
use hurl_core::ast::{Expr, ExprKind};

use super::function;
use crate::runner::error::{RunnerError, RunnerErrorKind};
use crate::runner::value::Value;
use crate::runner::VariableSet;

/// Evaluates the expression `expr` with `variables` map, returns a [`Value`] on success or an [`RunnerError`] .
pub fn eval(expr: &Expr, variables: &VariableSet) -> Result<Value, RunnerError> {
    match &expr.kind {
        ExprKind::Variable(variable) => {
            if let Some(variable) = variables.get(variable.name.as_str()) {
                Ok(variable.value().clone())
            } else {
                let kind = RunnerErrorKind::TemplateVariableNotDefined {
                    name: variable.name.clone(),
                };
                Err(RunnerError::new(variable.source_info, kind, false))
            }
        }
        ExprKind::Function(fct) => function::eval(fct),
    }
}

/// Render the expression `expr` with `variables` map, returns a [`String`] on success or an [`RunnerError`] .
pub fn render(expr: &Expr, variables: &VariableSet) -> Result<String, RunnerError> {
    let source_info = expr.source_info;
    let value = eval(expr, variables)?;
    if let Some(s) = value.render() {
        Ok(s)
    } else {
        let kind = RunnerErrorKind::UnrenderableExpression {
            value: value.to_string(),
        };
        Err(RunnerError::new(source_info, kind, false))
    }
}

#[cfg(test)]
mod tests {
    use hurl_core::ast::{ExprKind, SourceInfo, Variable};
    use hurl_core::reader::Pos;

    use super::*;

    #[test]
    fn test_render_expression() {
        let mut variables = VariableSet::new();
        variables
            .insert("status".to_string(), Value::Bool(true))
            .unwrap();
        let expr = Expr {
            kind: ExprKind::Variable(Variable {
                name: "status".to_string(),
                source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
            }),
            source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
        };
        assert_eq!(eval(&expr, &variables).unwrap(), Value::Bool(true));
        assert_eq!(render(&expr, &variables).unwrap(), "true");

        let data_chrono = chrono::DateTime::parse_from_rfc2822("Tue, 10 Jan 2023 08:29:52 GMT")
            .unwrap()
            .into();
        variables
            .insert("now".to_string(), Value::Date(data_chrono))
            .unwrap();
        let expr = Expr {
            kind: ExprKind::Variable(Variable {
                name: "now".to_string(),
                source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
            }),
            source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
        };
        assert_eq!(eval(&expr, &variables).unwrap(), Value::Date(data_chrono));
        assert_eq!(
            render(&expr, &variables).unwrap(),
            "2023-01-10T08:29:52.000000Z"
        );
    }
}
