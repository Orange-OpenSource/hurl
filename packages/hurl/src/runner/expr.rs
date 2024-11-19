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
use hurl_core::ast::Expr;

use crate::runner::error::{RunnerError, RunnerErrorKind};
use crate::runner::value::Value;
use crate::runner::VariableSet;

/// Evaluates the expression `expr` with `variables` map, returns a [`Value`] on success or an [`RunnerError`] .
pub fn eval(expr: &Expr, variables: &VariableSet) -> Result<Value, RunnerError> {
    if let Some(value) = variables.get(expr.variable.name.as_str()) {
        Ok(value.clone())
    } else {
        let kind = RunnerErrorKind::TemplateVariableNotDefined {
            name: expr.variable.name.clone(),
        };
        Err(RunnerError::new(expr.variable.source_info, kind, false))
    }
}

/// Render the expression `expr` with `variables` map, returns a [`String`] on success or an [`RunnerError`] .
pub fn render(expr: &Expr, variables: &VariableSet) -> Result<String, RunnerError> {
    let source_info = expr.variable.source_info;
    let name = &expr.variable.name;
    let value = eval(expr, variables)?;
    if value.is_renderable() {
        Ok(value.to_string())
    } else {
        let kind = RunnerErrorKind::UnrenderableVariable {
            name: name.to_string(),
            value: value.to_string(),
        };
        Err(RunnerError::new(source_info, kind, false))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hurl_core::{
        ast::{SourceInfo, Variable},
        reader::Pos,
    };

    #[test]
    fn test_render_expression() {
        let mut variables = VariableSet::new();
        variables.insert("status".to_string(), Value::Bool(true));
        let expr = Expr {
            variable: Variable {
                name: "status".to_string(),
                source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
            },
        };
        assert_eq!(eval(&expr, &variables).unwrap(), Value::Bool(true));
        assert_eq!(render(&expr, &variables).unwrap(), "true");
    }
}
