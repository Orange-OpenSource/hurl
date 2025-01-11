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
use crate::runner::{RunnerError, RunnerErrorKind, Value};
use hurl_core::ast::SourceInfo;
use std::collections::HashMap;

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
    variables: HashMap<String, Value>,
}

impl VariableSet {
    /// Creates a new empty set of variables.
    pub fn new() -> Self {
        VariableSet {
            variables: HashMap::new(),
        }
    }

    /// Creates a new variable set of public variable from an [`HashMap`].
    pub fn from(variables: &HashMap<String, Value>) -> Self {
        VariableSet {
            variables: variables.clone(),
        }
    }

    /// Inserts a variable named `name` with `value` into the variable set.
    ///
    /// This method fails when a secret value is being inserted whereas there is already a secret
    /// value with the same name as secret variables can't be overridden.
    pub fn insert(&mut self, name: String, value: Value) -> Result<(), Error> {
        // Secret values can't be overridden by public value, otherwise secret values
        // becomes public?
        if let Some(Value::Secret(_)) = self.variables.get(&name) {
            return Err(Error::ReadOnlySecret(name));
        }
        self.variables.insert(name, value);
        Ok(())
    }

    /// Returns a reference to the value corresponding to the variable named `name`.
    pub fn get(&self, name: &str) -> Option<&Value> {
        self.variables.get(name)
    }

    /// Returns an iterator over all the variables values.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Value)> {
        self.variables.iter()
    }

    /// Returns the list of all secrets values.
    pub fn secrets(&self) -> Vec<String> {
        self.variables
            .iter()
            .filter(|(_, value)| matches!(value, Value::Secret(_)))
            .map(|(_, value)| value.to_string())
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod test {
    use crate::runner::Number::{Float, Integer};
    use crate::runner::{Value, VariableSet};

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
            .insert("quic".to_string(), Value::Secret("42".to_string()))
            .unwrap();

        assert_eq!(
            variables.get("foo"),
            Some(&Value::String("xxx".to_string()))
        );
        assert!(variables.get("Foo").is_none());
        assert_eq!(variables.get("bar"), Some(&Value::Bool(true)));
        assert_eq!(variables.get("baz"), Some(&Value::Number(Float(1.0))));
        assert_eq!(
            variables.get("quic"),
            Some(&Value::Secret("42".to_string()))
        );
        assert!(variables.get("BAZ").is_none())
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
        for (name, value) in variables.iter() {
            let expected = expected_value(name, &data);
            assert_eq!(expected.unwrap(), value);
        }
    }

    #[test]
    fn secret_cant_be_reassigned() {
        let mut variables = VariableSet::new();
        variables
            .insert("foo".to_string(), Value::Secret("42".to_string()))
            .unwrap();
        assert!(variables
            .insert("foo".to_string(), Value::String("xxx".to_string()))
            .is_err());
    }

    #[test]
    fn get_secrets() {
        let mut variables = VariableSet::new();
        variables
            .insert("foo".to_string(), Value::Secret("42".to_string()))
            .unwrap();
        variables
            .insert("bar".to_string(), Value::String("toto".to_string()))
            .unwrap();
        variables
            .insert("baz".to_string(), Value::Bool(true))
            .unwrap();
        variables
            .insert("a".to_string(), Value::Secret("1234".to_string()))
            .unwrap();

        let mut secrets = variables.secrets();
        secrets.sort();
        assert_eq!(secrets, vec!["1234", "42"])
    }
}
