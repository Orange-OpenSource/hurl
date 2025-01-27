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
use std::fs;
use std::path::Path;

use serde_json::{Error, Value};

use crate::report::ReportError;
use crate::runner::HurlResult;

impl From<Error> for ReportError {
    fn from(value: Error) -> Self {
        ReportError::from_string(&value.to_string())
    }
}

pub fn parse_json_report(filename: &Path) -> Result<Vec<Value>, ReportError> {
    if !filename.exists() {
        return Ok(vec![]);
    }
    let s = match fs::read_to_string(filename) {
        Ok(s) => s,
        Err(e) => {
            return Err(ReportError::from_error(
                e,
                filename,
                "Issue reading JSON report",
            ))
        }
    };
    // TODO: if the existing JSON report is not valid, we consider that there is no
    // existing report to append, without displaying any error or warning. Maybe a better option
    // would be to raise an error here and ask the user to explicitly deal with this error.
    if let Ok(Some(Value::Array(values))) = serde_json::from_str(&s) {
        if values.iter().all(HurlResult::is_deserializable) {
            return Ok(values);
        }
    }
    Ok(vec![])
}
