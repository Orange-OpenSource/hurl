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
use hurl_core::types::Index;

/// This trait is implemented by run event observers, during the execution of one Hurl file.
pub trait EventListener {
    /// Call when running an entry.
    /// `current` is the entry index in the Hurl file,
    /// `last` is the last entry index (may be less that the total number of entries).
    /// `retry_count` is the current number of retries (i.e. 0 for a first run)
    fn on_entry_running(&self, current: Index, last: Index, retry_count: usize);
}
