/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2026 Orange
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

use hurl::runner::Value;

use super::CliOptionsError;

/// Add a secret with `name` and `value` to the `secrets` hash map.
pub fn add_secret(
    secrets: &mut HashMap<String, String>,
    name: String,
    value: Value,
) -> Result<(), CliOptionsError> {
    // We check that there is no existing secret with this name
    if secrets.contains_key(&name) {
        return Err(CliOptionsError::Error(format!(
            "secret '{}' can't be reassigned",
            &name
        )));
    }
    let Value::String(value) = value else {
        panic!("Secrets must be string");
    };
    secrets.insert(name.to_string(), value);
    Ok(())
}
