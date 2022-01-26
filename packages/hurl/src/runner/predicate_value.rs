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
use std::collections::HashMap;

use hurl_core::ast::*;

use super::core::Error;
use super::expr::eval_expr;
use super::template::eval_template;
use super::value::Value;

pub fn eval_predicate_value(
    predicate_value: PredicateValue,
    variables: &HashMap<String, Value>,
) -> Result<Value, Error> {
    match predicate_value {
        PredicateValue::String(template) => {
            let s = eval_template(&template, variables)?;
            Ok(Value::String(s))
        }
        PredicateValue::Raw(value) => {
            let s = eval_template(&value.value, variables)?;
            Ok(Value::String(s))
        }
        PredicateValue::Integer(value) => Ok(Value::Integer(value)),
        PredicateValue::Float(value) => Ok(Value::Float(value.value)),
        PredicateValue::Bool(value) => Ok(Value::Bool(value)),
        PredicateValue::Null {} => Ok(Value::Null {}),
        PredicateValue::Hex(value) => Ok(Value::Bytes(value.value)),
        PredicateValue::Base64(value) => Ok(Value::Bytes(value.value)),
        PredicateValue::Expression(expr) => {
            let value = eval_expr(expr, variables)?;
            Ok(value)
        }
        PredicateValue::Regex(regex) => Ok(Value::Regex(regex.inner)),
    }
}
