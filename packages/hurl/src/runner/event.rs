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
/// This trait is implemented by run event observers, during the execution of one Hurl file.
pub trait EventListener {
    /// Call when running an entry, `entry_index` is the entry 0-based index in the Hurl file,
    /// and `entry_count` is the total number of entries in the Hurl file.
    fn on_running(&self, entry_index: usize, entry_count: usize);
}
