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

/// Function extensions:
/// <https://www.rfc-editor.org/rfc/rfc9535.html#name-function-extensions>
/// This module implements the functions defined in the RFC.
/// It defines the function types and their arguments.
pub mod argument;
pub mod functions;

use crate::jsonpath2::eval::NodeList;

pub type ValueType = Option<serde_json::Value>;
pub type LogicalType = bool;
pub type NodesType = NodeList;
