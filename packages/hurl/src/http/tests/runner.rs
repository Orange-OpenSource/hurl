/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2023 Orange
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

use crate::runner;
use crate::runner::RunnerOptions;
use crate::util::logger::LoggerOptionsBuilder;

#[test]
fn test_hello() {
    let content = "GET http://localhost;8000/hello";
    let logger_opts = LoggerOptionsBuilder::new().build();
    let variables = HashMap::new();
    let runner_opts = RunnerOptions::default();
    runner::run(content, &runner_opts, &variables, &logger_opts).unwrap();
}
