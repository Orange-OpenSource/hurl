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

use super::variables::TypeKind;
use super::{secret, variables, CliOptions, CliOptionsError, RunContext};

/// Parses Hurl configuration defined in environment variables.
pub fn parse_env_vars(
    context: &RunContext,
    default_options: CliOptions,
) -> Result<CliOptions, CliOptionsError> {
    let mut options = default_options;
    options.variables = parse_variables(context, options.variables)?;
    options.secrets = parse_secrets(context, options.secrets)?;
    Ok(options)
}

/// Parses Hurl variables configured in environment variables, given a set of existing variables
/// `default_variables`.
///
/// Variables can be set with `HURL_VARIABLE_foo` and `HURL_foo` (legacy syntax).
fn parse_variables(
    context: &RunContext,
    default_variables: HashMap<String, Value>,
) -> Result<HashMap<String, Value>, CliOptionsError> {
    let mut variables = default_variables;

    // Variables are typed, based on their values.
    let type_kind = TypeKind::Inferred;

    // Insert environment variables `HURL_VARIABLE_foo`
    for (env_name, env_value) in context.var_env_vars() {
        let value = variables::parse_value(env_value, type_kind)?;
        variables.insert(env_name.to_string(), value);
    }

    // Insert legacy environment variables `HURL_foo`
    for (env_name, env_value) in context.legacy_var_env_vars() {
        let value = variables::parse_value(env_value, type_kind)?;
        variables.insert(env_name.to_string(), value);
    }

    Ok(variables)
}

/// Parses Hurl secrets configured in environment variables, given a set of existing secrets
/// `default_secrets`.
///
/// Secrets can be set with `HURL_SECRET_foo`.
fn parse_secrets(
    context: &RunContext,
    default_secrets: HashMap<String, String>,
) -> Result<HashMap<String, String>, CliOptionsError> {
    let mut secrets = default_secrets;

    // Secrets are always parsed as string.
    let type_kind = TypeKind::String;

    // Insert environment secrets `HURL_SECRET_foo`
    for (env_name, env_value) in context.secret_env_vars() {
        let value = variables::parse_value(env_value, type_kind)?;
        secret::add_secret(&mut secrets, env_name.to_string(), value)?;
    }
    Ok(secrets)
}

#[cfg(test)]
mod tests {
    use super::{parse_env_vars, CliOptions, RunContext};
    use hurl::runner::{Number, Value};
    use std::collections::HashMap;

    #[test]
    fn test_options_variables_override_by_env_vars() {
        let stdin_term = true;
        let stdout_term = true;
        let stderr_term = true;

        // Default configuration of Hurl run.
        let mut options = CliOptions::default();
        let mut variables = HashMap::new();
        variables.insert("var1".to_string(), Value::String("zzz".to_string()));
        variables.insert("foo".to_string(), Value::String("FOO".to_string()));
        options.variables = variables;

        // Overrides Hurl run variables with env vars.
        let env_vars_override = HashMap::from([
            ("FOO".to_string(), "xxx".to_string()),
            ("HURL_VARIABLE_foo".to_string(), "48".to_string()),
            ("HURL_VARIABLE_bar".to_string(), "BAR".to_string()),
            ("HURL_baz".to_string(), "abcd".to_string()),
            ("NOT_A_VARIABLE".to_string(), "bar".to_string()),
        ]);
        let ctx = RunContext::new(env_vars_override, stdin_term, stdout_term, stderr_term);

        let updated_options = parse_env_vars(&ctx, options).unwrap();
        assert_eq!(updated_options.variables.len(), 4);
        assert_eq!(
            updated_options.variables["foo"],
            Value::Number(Number::Integer(48))
        );
        assert_eq!(
            updated_options.variables["var1"],
            Value::String("zzz".to_string())
        );
        assert_eq!(
            updated_options.variables["bar"],
            Value::String("BAR".to_string())
        );
        assert_eq!(
            updated_options.variables["baz"],
            Value::String("abcd".to_string())
        );
    }

    #[test]
    fn test_options_secrets_override_by_env_vars() {
        let stdin_term = true;
        let stdout_term = true;
        let stderr_term = true;

        // Default configuration of Hurl run.
        let mut options = CliOptions::default();
        let mut secrets = HashMap::new();
        secrets.insert("secret1".to_string(), "SECRET1".to_string());
        options.secrets = secrets;

        // Overrides Hurl run secrets with env vars.
        let env_vars_override = HashMap::from([
            ("QUX".to_string(), "qux".to_string()),
            ("HURL_SECRET_secret2".to_string(), "SECRET2".to_string()),
            ("HURL_VARIABLE_bar".to_string(), "BAR".to_string()),
            ("HURL_SECRET_secret3".to_string(), "SECRET3".to_string()),
        ]);
        let ctx = RunContext::new(env_vars_override, stdin_term, stdout_term, stderr_term);

        let updated_options = parse_env_vars(&ctx, options).unwrap();
        assert_eq!(updated_options.variables.len(), 1);
        assert_eq!(
            updated_options.variables["bar"],
            Value::String("BAR".to_string())
        );
        assert_eq!(updated_options.secrets.len(), 3);
        assert_eq!(updated_options.secrets["secret1"], "SECRET1".to_string(),);
        assert_eq!(updated_options.secrets["secret2"], "SECRET2".to_string(),);
        assert_eq!(updated_options.secrets["secret3"], "SECRET3".to_string(),);
    }
}
