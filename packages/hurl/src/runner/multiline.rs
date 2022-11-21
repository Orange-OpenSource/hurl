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
use crate::runner::template::eval_template;
use crate::runner::{Error, Value};
use hurl_core::ast::{GraphQl, MultilineString, Text};
use serde_json::json;
use std::collections::HashMap;

pub fn eval_multiline(
    multiline: &MultilineString,
    variables: &HashMap<String, Value>,
) -> Result<String, Error> {
    match multiline {
        MultilineString::TextOneline(value) => {
            let s = eval_template(value, variables)?;
            Ok(s)
        }
        MultilineString::Text(Text { value, .. })
        | MultilineString::Json(Text { value, .. })
        | MultilineString::Xml(Text { value, .. }) => {
            let s = eval_template(value, variables)?;
            Ok(s)
        }
        MultilineString::GraphQl(GraphQl { value, .. }) => {
            let query = eval_template(value, variables)?;
            let body = json!({ "query": query });
            Ok(body.to_string())
        }
    }
}
