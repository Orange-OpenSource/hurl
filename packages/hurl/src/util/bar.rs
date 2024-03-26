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

/// Returns the progress bar with the 1-based current `index`.
pub fn progress_bar(index: usize, count: usize) -> String {
    const WIDTH: usize = 24;
    // We report the number of items already processed.
    let progress = (index - 1) as f64 / count as f64;
    let col = (progress * WIDTH as f64) as usize;
    let completed = if col > 0 {
        "=".repeat(col)
    } else {
        String::new()
    };
    let void = " ".repeat(WIDTH - col - 1);
    format!("[{completed}>{void}] {index}/{count}")
}

#[cfg(test)]
mod tests {
    use crate::util::bar::progress_bar;

    #[rustfmt::skip]
    #[test]
    fn test_progress_bar() {
        // Progress strings with 20 entries:
        assert_eq!(progress_bar(1, 20),  "[>                       ] 1/20");
        assert_eq!(progress_bar(2, 20),  "[=>                      ] 2/20");
        assert_eq!(progress_bar(5, 20),  "[====>                   ] 5/20");
        assert_eq!(progress_bar(10, 20), "[==========>             ] 10/20");
        assert_eq!(progress_bar(15, 20), "[================>       ] 15/20");
        assert_eq!(progress_bar(20, 20), "[======================> ] 20/20");

        // Progress strings with 3 entries:
        assert_eq!(progress_bar(1, 3), "[>                       ] 1/3");
        assert_eq!(progress_bar(2, 3), "[========>               ] 2/3");
        assert_eq!(progress_bar(3, 3), "[================>       ] 3/3");

        // Progress strings with 1 entries:
        assert_eq!(progress_bar(1, 1), "[>                       ] 1/1");
    }
}
