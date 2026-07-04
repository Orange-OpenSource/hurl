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
use std::time::Duration;

use hurl_core::types::{self, DurationUnit};

use super::CliOptionsError;

/// Parses a string with or without time unit into a `Duration`.
///
/// When there is no time unit in the user string, the duration is parsed with a default time unit.
///
/// Example: `32s`, `10m`, `20000`.
///
pub fn duration_from_str(
    value: &str,
    default_unit: DurationUnit,
) -> Result<Duration, CliOptionsError> {
    types::duration_from_str(value, default_unit).map_err(CliOptionsError::Error)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_parse_error() {
        assert_eq!(
            duration_from_str("", DurationUnit::MilliSecond).unwrap_err(),
            CliOptionsError::Error("Invalid duration".to_string())
        );
        assert_eq!(
            duration_from_str("s", DurationUnit::MilliSecond).unwrap_err(),
            CliOptionsError::Error("Invalid duration".to_string())
        );
        assert_eq!(
            duration_from_str("10s10", DurationUnit::MilliSecond).unwrap_err(),
            CliOptionsError::Error("Invalid duration".to_string())
        );
        assert_eq!(
            duration_from_str("10mm", DurationUnit::MilliSecond).unwrap_err(),
            CliOptionsError::Error("Invalid duration unit mm".to_string())
        );
        assert_eq!(
            duration_from_str("18446744073709551616", DurationUnit::MilliSecond).unwrap_err(),
            CliOptionsError::Error("Duration value too large".to_string())
        );
        assert_eq!(
            duration_from_str("18446744073709551615s", DurationUnit::MilliSecond).unwrap_err(),
            CliOptionsError::Error("Duration value too large".to_string())
        );
    }

    #[test]
    pub fn test_parse() {
        assert_eq!(
            duration_from_str("10", DurationUnit::MilliSecond).unwrap(),
            Duration::from_millis(10)
        );
        assert_eq!(
            duration_from_str("10s", DurationUnit::MilliSecond).unwrap(),
            Duration::from_secs(10)
        );
        assert_eq!(
            duration_from_str("10000ms", DurationUnit::Second).unwrap(),
            Duration::from_millis(10000)
        );
        assert_eq!(
            duration_from_str("5m", DurationUnit::Second).unwrap(),
            Duration::from_secs(5 * 60)
        );
        assert_eq!(
            duration_from_str("3h", DurationUnit::MilliSecond).unwrap(),
            Duration::from_hours(3)
        );
    }
}
