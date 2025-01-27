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

//! JSONPath specs.
//!
//! There is no proper specifications for JSONPath.
//! The de-facto one is still <https://goessner.net/articles/JsonPath/>.
//!
//! Hurl will try to follow this one as closely as possible
//!
//! There are a few edge cases for which several implementations differ.
//!
//! We describe below the behaviour that we expect in Hurl.
//!
//! Specify a field key in a subscript operator:  `$['name']`.
//! The key must be enclosed within single quotes only.
//! The following expressions will not be valid: `$["name"]` and `$[name]`.
//!
//! Accessing a key containing a single quote must be escape:  `$['\'']`.
//! Key with unicode are supported: `$['✈']`
//!
//! Any character within these quote won't have a specific meaning:
//! - `$['*']` selects the element with key '*'. It is different from `$[*]` which selects all elements
//! - `$['.']` selects the element with key '.'.
//!
//! The dot notation is usually more readable the bracket notation
//! but it is more limited in terms of allowed characters.
//! The following characters are allowed:
//!   - alphanumeric
//!   - _ (underscore)
//!
//! Filters can be applied to element of an array with the `?(@.key PREDICATE)` notation.
//! The key can specify one or more levels.
//! For example, `.price.US` specify field 'US' in an object for the field price.
//! The predicate if not present just checks the key existence.
//!
//! The Hurl API for evaluating a jsonpath expression does not always return a collection (as defined in the jsonpath spec).
//! It returns an optional value, which is either a collection or a single value (scalar).
//! Note that other implementations (such as the Java lib <https://github.com/json-path/JsonPath>) also distinguish between node value (definite path) and collection (indefinite path).
//!
//! Note that the only selectors returning a scalar are:
//! - array index selector (`$.store.book[2]`)
//! - object key selector (`$.store.bicycle.color/$.store.bicycle['color']`)
//!  
//! This will make testing the value a bit easier.
//!

pub use self::eval::JsonpathResult;
pub use self::parser::parse;

mod ast;
mod eval;
mod parser;

#[cfg(test)]
mod tests;
