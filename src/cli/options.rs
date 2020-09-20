/*
 * hurl (https://hurl.dev)
 * Copyright (C) 2020 Orange
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

use super::Error;

pub fn cookies_output_file(filename: String, n: usize) -> Result<std::path::PathBuf, Error> {
    if n > 1 {
        Err(Error {
            message: "Only save cookies for a unique session".to_string(),
        })
    } else {
        let path = std::path::Path::new(&filename);
        Ok(path.to_path_buf())
    }
}

pub fn output_color(color_present: bool, no_color_present: bool, stdout: bool) -> bool {
    if color_present {
        true
    } else if no_color_present {
        false
    } else {
        stdout
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_color() {
        assert_eq!(output_color(true, false, true), true);
        assert_eq!(output_color(false, false, true), true);
    }
}
