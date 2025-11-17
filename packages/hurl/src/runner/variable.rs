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
use std::collections::{HashMap, HashSet};

use super::value::Value;

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

/// Represents a set of variables, either injected at the start
/// of execution, or inserted during a run.
#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct VariableSet {
    /// The map of variables
    variables: HashMap<String, Variable>,
    /// All the secrets values: the current secrets, as values of current secret variables,
    /// and the previous secrets. What's a secret remains a secret forever, even if a new secret
    /// variable get a new value.
    secrets: HashSet<String>,
}

impl VariableSet {
    /// Creates a new empty set of variables.
    pub fn new() -> Self {
        VariableSet {
            variables: HashMap::new(),
            secrets: HashSet::new(),
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
            secrets: HashSet::new(),
        }
    }

    /// Inserts a public variable named `name` with `value`.
    pub fn insert(&mut self, name: String, value: Value) {
        let variable = Variable::new(&name, &value, Visibility::Public);
        self.variables.insert(name, variable);
    }

    /// Inserts a secret string value named `name` with `value`.
    ///
    /// A secret is declaring also a new secret variable named `name` with `value` so this secret
    /// can be used directly as a variable in a Hurl file.
    pub fn insert_secret(&mut self, name: String, value: String) {
        // Update our secrets list with this new value
        self.secrets.insert(value.clone());
        let value = Value::String(value);
        let variable = Variable::new(&name, &value, Visibility::Secret);
        self.variables.insert(name, variable);
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
        self.secrets.iter().cloned().collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod test {
    use crate::runner::Number::{Float, Integer};
    use crate::runner::{Value, Variable, VariableSet, Visibility};

    #[test]
    fn simple_variable_set() {
        let mut variables = VariableSet::new();

        variables.insert("foo".to_string(), Value::String("xxx".to_string()));
        variables.insert("bar".to_string(), Value::Number(Integer(42)));
        variables.insert("bar".to_string(), Value::Bool(true));
        variables.insert("baz".to_string(), Value::Number(Float(1.0)));
        variables.insert_secret("quic".to_string(), "42".to_string());

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

        assert_eq!(variables.secrets().len(), 1);
        assert!(variables.secrets().contains(&"42".to_string()));
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
            variables.insert(name, value);
        });

        // Test iter()
        for (name, variable) in variables.iter() {
            let expected = expected_value(name, &data);
            assert_eq!(expected.unwrap(), &variable.value);
        }
    }

    #[test]
    fn secret_variable_can_be_reassigned_but_remains_secret() {
        let mut variables = VariableSet::new();
        variables.insert_secret("foo".to_string(), "42".to_string());
        variables.insert("foo".to_string(), Value::String("xxx".to_string()));

        assert_eq!(
            variables.get("foo").unwrap().value,
            Value::String("xxx".to_string())
        );
        assert!(variables.secrets().contains(&"42".to_string()));
    }

    #[test]
    fn get_secrets() {
        let mut variables = VariableSet::new();
        variables.insert_secret("foo".to_string(), "42".to_string());
        variables.insert("bar".to_string(), Value::String("toto".to_string()));
        variables.insert("baz".to_string(), Value::Bool(true));
        variables.insert_secret("a".to_string(), "1234".to_string());

        let mut secrets = variables.secrets();
        secrets.sort();
        assert_eq!(secrets, vec!["1234", "42"]);
    }
}
