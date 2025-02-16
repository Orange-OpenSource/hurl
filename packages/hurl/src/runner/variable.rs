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
use std::collections::HashMap;

use hurl_core::ast::SourceInfo;

use crate::runner::{RunnerError, RunnerErrorKind, Value};

/// Represents a variable named to hold `Value`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Variable {
    /// Name of this variable.
    name: String,
    /// Value of this variable.
    value: Value,
    /// A variable is either public, or secret.
    visibility: Visibility,
}

/// Visibility of a variable value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Visibility {
    /// The variable's value is always visible.
    Public,
    /// The variable's value is redacted from standard error and reports.
    Secret,
}

impl Variable {
    /// Creates a new variable named `name` with this `value` and `visibility`.
    pub fn new(name: &str, value: &Value, visibility: Visibility) -> Self {
        Variable {
            name: name.to_string(),
            value: value.clone(),
            visibility,
        }
    }

    /// Returns a reference to this variable's `value`.
    pub fn value(&self) -> &Value {
        &self.value
    }

    /// Returns `true` if this variable is secret.
    pub fn is_secret(&self) -> bool {
        matches!(self.visibility, Visibility::Secret)
    }
}

/// Errors raised when trying to insert a public/secret variable into a [`VariableSet`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    ReadOnlySecret(String),
}

impl Error {
    /// Converts an instance of [`Error`] to a [`RunnerError`].
    pub fn to_runner_error(&self, source_info: SourceInfo) -> RunnerError {
        let Error::ReadOnlySecret(name) = self;
        let kind = RunnerErrorKind::ReadOnlySecret { name: name.clone() };
        RunnerError::new(source_info, kind, false)
    }
}

/// Represents a set of variables, either injected at the start
/// of execution, or inserted during a run.
#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct VariableSet {
    variables: HashMap<String, Variable>,
}

impl VariableSet {
    /// Creates a new empty set of variables.
    pub fn new() -> Self {
        VariableSet {
            variables: HashMap::new(),
        }
    }

    /// Creates a new set of public variables from a [`HashMap`].
    pub fn from(variables: &HashMap<String, Value>) -> Self {
        let variables = variables
            .iter()
            .map(|(name, value)| {
                (
                    name.to_string(),
                    Variable::new(name, value, Visibility::Public),
                )
            })
            .collect::<HashMap<_, _>>();
        VariableSet {
            variables: variables.clone(),
        }
    }

    /// Inserts a public variable named `name` with `value`
    ///
    /// This method fails when a secret value is being inserted whereas there is already a secret
    /// value with the same name as secret variables can't be overridden.
    pub fn insert(&mut self, name: String, value: Value) -> Result<(), Error> {
        // Secret variables can't be overridden by public variables, otherwise secret variables values
        // becomes public.
        if let Some(Variable {
            visibility: Visibility::Secret,
            ..
        }) = self.variables.get(&name)
        {
            return Err(Error::ReadOnlySecret(name.clone()));
        }
        let variable = Variable::new(&name, &value, Visibility::Public);
        self.variables.insert(name, variable);
        Ok(())
    }

    /// Inserts a secret string value named `name` with `value`.
    ///
    /// This method fails when a secret value is being inserted whereas there is already a secret
    /// value with the same name as secret variables can't be overridden.
    pub fn insert_secret(&mut self, name: String, value: String) -> Result<(), Error> {
        // Secret variables can't be overridden by public variables, otherwise secret variables values
        // becomes public.
        if let Some(Variable {
            visibility: Visibility::Secret,
            ..
        }) = self.variables.get(&name)
        {
            return Err(Error::ReadOnlySecret(name.clone()));
        }
        let value = Value::String(value);
        let variable = Variable::new(&name, &value, Visibility::Secret);
        self.variables.insert(name, variable);
        Ok(())
    }

    /// Returns a reference to the value corresponding to the variable named `name`.
    pub fn get(&self, name: &str) -> Option<&Variable> {
        self.variables.get(name)
    }

    /// Returns an iterator over all the variables values.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Variable)> {
        self.variables.iter()
    }

    /// Returns the list of all secrets values.
    pub fn secrets(&self) -> Vec<String> {
        self.variables
            .iter()
            .filter(|(_, variable)| variable.is_secret())
            .map(|(_, variable)| variable.value.to_string())
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod test {
    use crate::runner::Number::{Float, Integer};
    use crate::runner::{Value, Variable, VariableSet, Visibility};

    #[test]
    fn simple_variable_set() {
        let mut variables = VariableSet::new();

        variables
            .insert("foo".to_string(), Value::String("xxx".to_string()))
            .unwrap();
        variables
            .insert("bar".to_string(), Value::Number(Integer(42)))
            .unwrap();
        variables
            .insert("bar".to_string(), Value::Bool(true))
            .unwrap();
        variables
            .insert("baz".to_string(), Value::Number(Float(1.0)))
            .unwrap();
        variables
            .insert_secret("quic".to_string(), "42".to_string())
            .unwrap();

        assert_eq!(
            variables.get("foo"),
            Some(&Variable::new(
                "foo",
                &Value::String("xxx".to_string()),
                Visibility::Public
            ))
        );
        assert!(variables.get("Foo").is_none());
        assert_eq!(
            variables.get("bar"),
            Some(&Variable::new(
                "bar",
                &Value::Bool(true),
                Visibility::Public
            ))
        );
        assert_eq!(
            variables.get("baz"),
            Some(&Variable::new(
                "baz",
                &Value::Number(Float(1.0)),
                Visibility::Public
            ))
        );
        assert_eq!(
            variables.get("quic"),
            Some(&Variable::new(
                "quic",
                &Value::String("42".to_string()),
                Visibility::Secret
            ))
        );
        assert!(variables.get("BAZ").is_none());
    }

    #[test]
    fn iter_variable_set() {
        fn expected_value<'data>(
            name: &str,
            data: &'data [(String, Value)],
        ) -> Option<&'data Value> {
            for (n, v) in data.iter() {
                if n == name {
                    return Some(v);
                }
            }
            None
        }

        let data = [
            ("foo".to_string(), Value::String("yyy".to_string())),
            ("bar".to_string(), Value::Bool(false)),
            ("baz".to_string(), Value::Number(Float(12.0))),
        ];
        let mut variables = VariableSet::new();
        data.clone().into_iter().for_each(|(name, value)| {
            variables.insert(name, value).unwrap();
        });

        // Test iter()
        for (name, variable) in variables.iter() {
            let expected = expected_value(name, &data);
            assert_eq!(expected.unwrap(), &variable.value);
        }
    }

    #[test]
    fn secret_cant_be_reassigned() {
        let mut variables = VariableSet::new();
        variables
            .insert_secret("foo".to_string(), "42".to_string())
            .unwrap();
        assert!(variables
            .insert("foo".to_string(), Value::String("xxx".to_string()))
            .is_err());
    }

    #[test]
    fn get_secrets() {
        let mut variables = VariableSet::new();
        variables
            .insert_secret("foo".to_string(), "42".to_string())
            .unwrap();
        variables
            .insert("bar".to_string(), Value::String("toto".to_string()))
            .unwrap();
        variables
            .insert("baz".to_string(), Value::Bool(true))
            .unwrap();
        variables
            .insert_secret("a".to_string(), "1234".to_string())
            .unwrap();

        let mut secrets = variables.secrets();
        secrets.sort();
        assert_eq!(secrets, vec!["1234", "42"]);
    }
}
