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

use crate::http::{Request, Response, Timings};

/// Holds an HTTP request and the corresponding HTTP response.
/// The request and responses are the runtime, evaluated data created by an HTTP exchange.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Call {
    /// The real HTTP request (vs the specified request in a Hurl file source)
    pub request: Request,
    /// The real HTTP response (vs the specified request in a Hurl file source)
    pub response: Response,
    /// Timings of the exchange, see <https://hurl.dev/docs/response.html#timings>
    pub timings: Timings,
}
