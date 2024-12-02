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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum VariableKind {
    /// A public variable.
    Public,
    /// A private variable, holding a secret value.
    Secret,
}

/// Represents a variable, holding a [`Value`].
#[derive(Clone, Debug, Eq, PartialEq)]
struct Variable {
    value: Value,
    kind: VariableKind,
}

impl Variable {
    /// Creates a new variable with `value` and `kind`.
    fn new(value: Value, kind: VariableKind) -> Self {
        Variable { value, kind }
    }
}

/// Errors raised when trying to insert a public/secret variable into a [`VariableSet`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InsertError {
    ReadOnlySecret(String),
}

impl InsertError {
    /// Converts an instance of [`InsertError`] to a [`RunnerError`].
    pub fn to_runner_error(&self, source_info: SourceInfo) -> RunnerError {
        let InsertError::ReadOnlySecret(name) = self;
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

    /// Creates a new variable set of public variable from an [`HashMap`].
    pub fn from(variables: &HashMap<String, Value>) -> Self {
        let variables = variables
            .iter()
            .map(|(name, value)| {
                (
                    name.to_string(),
                    Variable::new(value.clone(), VariableKind::Public),
                )
            })
            .collect::<HashMap<_, _>>();
        VariableSet { variables }
    }

    /// Inserts a public variable named `name` with `value` into the variable set.
    ///
    /// This method fails when there is a secret variable in the variable set as secret variables
    /// can't be overridden.
    pub fn insert(&mut self, name: String, value: Value) -> Result<(), InsertError> {
        // Secret values can't be overridden by public value, otherwise secret values
        // becomes public?
        if let Some(Variable {
            kind: VariableKind::Secret,
            ..
        }) = self.variables.get(&name)
        {
            return Err(InsertError::ReadOnlySecret(name));
        }
        let variable = Variable::new(value, VariableKind::Public);
        self.variables.insert(name, variable);
        Ok(())
    }

    #[deprecated(
        note = "This method is not yet ready for use: secret/private variables are still under development"
    )]
    /// Inserts a secret variable named `name` with `value` into the variable set.
    ///
    /// Contrary to [`VariableSet::insert`], this method can not fail: a secret can override a
    /// public variable and a secret variable.
    pub fn insert_secret(&mut self, name: String, value: Value) {
        let variable = Variable::new(value, VariableKind::Secret);
        self.variables.insert(name, variable);
    }

    /// Returns a reference to the value corresponding to the variable named `name`.
    pub fn get(&self, name: &str) -> Option<&Value> {
        self.variables.get(name).map(|v| &v.value)
    }

    /// Returns true if the variable set contains no variables.
    pub fn is_empty(&self) -> bool {
        self.variables.is_empty()
    }

    /// Returns an iterator over all the variables values.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Value)> {
        self.variables
            .iter()
            .map(|(name, variable)| (name, &variable.value))
    }

    /// Returns the number of variables in the set.
    pub fn len(&self) -> usize {
        self.variables.len()
    }
}

#[cfg(test)]
mod test {
    use crate::runner::Number::{Float, Integer};
    use crate::runner::{Value, VariableSet};

    #[allow(deprecated)]
    #[test]
    fn simple_variable_set() {
        let mut variables = VariableSet::new();
        assert!(variables.is_empty());

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
        variables.insert_secret("quic".to_string(), Value::Number(Integer(42)));

        assert_eq!(variables.len(), 4);

        assert_eq!(
            variables.get("foo"),
            Some(&Value::String("xxx".to_string()))
        );
        assert_eq!(variables.get("bar"), Some(&Value::Bool(true)));
        assert_eq!(variables.get("baz"), Some(&Value::Number(Float(1.0))));
        assert_eq!(variables.get("quic"), Some(&Value::Number(Integer(42))));
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

    #[allow(deprecated)]
    #[test]
    fn secret_cant_be_reassigned() {
        let mut variables = VariableSet::new();
        variables.insert_secret("foo".to_string(), Value::Number(Integer(42)));
        assert!(variables
            .insert("foo".to_string(), Value::String("xxx".to_string()))
            .is_err());
    }
}
