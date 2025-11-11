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

pub(crate) mod comparison;
pub(crate) mod expr;
pub(crate) mod literal;
pub(crate) mod query;
pub(crate) mod segment;
pub(crate) mod selector;
pub(crate) mod singular_query;

/// JSONPath Query
/// https://www.rfc-editor.org/rfc/rfc9535.html#name-overview-of-jsonpath-expres
/// This is the standard JSONPath query used outside the module
#[allow(dead_code)]
pub(crate) type JsonPathQuery = query::AbsoluteQuery;
