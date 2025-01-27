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
use crate::http::Timings;
use crate::util::logger::Logger;

impl Timings {
    /// Logs the response timings information.
    pub fn log(&self, logger: &mut Logger) {
        logger.debug_important("Timings:");
        logger.debug(&format!("begin: {}", self.begin_call));
        logger.debug(&format!("end: {}", self.end_call));
        logger.debug(&format!("namelookup: {} µs", self.name_lookup.as_micros()));
        logger.debug(&format!("connect: {} µs", self.connect.as_micros()));
        logger.debug(&format!("app_connect: {} µs", self.app_connect.as_micros()));
        logger.debug(&format!(
            "pre_transfer: {} µs",
            self.pre_transfer.as_micros()
        ));
        logger.debug(&format!(
            "start_transfer: {} µs",
            self.start_transfer.as_micros()
        ));
        logger.debug(&format!("total: {} µs", self.total.as_micros()));
    }
}
