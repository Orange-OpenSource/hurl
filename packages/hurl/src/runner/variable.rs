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
use crate::runner::Value;
use std::collections::HashMap;

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

    /// Creates a new variable set from an [`HashMap`].
    pub fn from(variables: &HashMap<String, Value>) -> Self {
        VariableSet {
            variables: variables.clone(),
        }
    }

    /// Inserts a variable named `name` with `value` into the variable set.
    ///
    /// If the variable set did not have this key present, [`None`] is returned.
    ///
    /// If the variable set did have this key present, the value is updated, and the old
    /// value is returned.
    pub fn insert(&mut self, name: String, value: Value) -> Option<Value> {
        self.variables.insert(name, value)
    }

    /// Returns a reference to the value corresponding to the variable named `name`.
    pub fn get(&self, name: &str) -> Option<&Value> {
        self.variables.get(name)
    }

    /// Returns true if the variable set contains no variables.
    pub fn is_empty(&self) -> bool {
        self.variables.is_empty()
    }

    /// Returns an iterator over all the variables.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Value)> {
        self.variables.iter()
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

    #[test]
    fn test_simple_variable_set() {
        let mut variables = VariableSet::new();
        assert!(variables.is_empty());

        variables.insert("foo".to_string(), Value::String("xxx".to_string()));
        variables.insert("bar".to_string(), Value::Number(Integer(42)));
        variables.insert("bar".to_string(), Value::Bool(true));
        variables.insert("baz".to_string(), Value::Number(Float(1.0)));

        assert_eq!(variables.len(), 3);

        assert_eq!(
            variables.get("foo"),
            Some(&Value::String("xxx".to_string()))
        );
        assert_eq!(variables.get("bar"), Some(&Value::Bool(true)));
        assert_eq!(variables.get("baz"), Some(&Value::Number(Float(1.0))));
        assert!(variables.get("BAZ").is_none())
    }

    #[test]
    fn test_iter_variable_set() {
        fn expected_value<'data>(
            data: &'data [(String, Value)],
            name: &str,
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
        for v in variables.iter() {
            let expected = expected_value(&data, v.0);
            assert_eq!(expected.unwrap(), v.1);
        }
    }
}
