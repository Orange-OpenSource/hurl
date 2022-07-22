/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2022 Orange
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

use crate::util;
use std::path::{Path, PathBuf};

/// Represents the directories used to run a Hurl file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContextDir {
    /// The current working directory.
    /// If current directory is a relative path, the `is_allowed` is not guaranteed to be correct.
    current_dir: PathBuf,
    /// The file root, either inferred or explicitly positioned by the user.
    /// As a consequence, it is always defined (and can't be replaced by a `Option<PathBuf>`).
    /// It can be relative (to the current directory) or absolute.
    file_root: PathBuf,
}

impl Default for ContextDir {
    fn default() -> Self {
        ContextDir {
            current_dir: PathBuf::new(),
            file_root: PathBuf::new(),
        }
    }
}

impl ContextDir {
    /// Returns a context directory with the given current directory and file root.
    pub fn new(current_dir: &Path, file_root: &Path) -> ContextDir {
        ContextDir {
            current_dir: PathBuf::from(current_dir),
            file_root: PathBuf::from(file_root),
        }
    }

    /// Returns a path (absolute or relative), given a filename.
    pub fn get_path(&self, filename: &str) -> PathBuf {
        self.file_root.join(Path::new(filename))
    }

    /// Checks if a given filename access is authorized.
    /// This method is used to check if a local file can be included in POST request.
    pub fn is_access_allowed(&self, filename: &str) -> bool {
        let file = self.get_path(filename);
        let absolute_file = self.current_dir.join(file);
        let absolute_file_root = self.current_dir.join(&self.file_root);
        util::is_descendant(absolute_file.as_path(), absolute_file_root.as_path())
    }
}

#[cfg(test)]
mod tests {
    use crate::http::ContextDir;
    use std::path::Path;

    #[test]
    fn check_filename_allowed_access_without_user_file_root() {
        // ```
        // $ cd /tmp
        // $ hurl test.hurl
        // ```
        let current_dir = Path::new("/tmp");
        let file_root = Path::new("");
        let context_dir = ContextDir::new(current_dir, file_root);
        assert!(context_dir.is_access_allowed("foo.bin"));
        assert!(context_dir.is_access_allowed("/tmp/foo.bin"));
        assert!(context_dir.is_access_allowed("a/foo.bin"));
        assert!(context_dir.is_access_allowed("a/b/foo.bin"));
        assert!(context_dir.is_access_allowed("../tmp/a/b/foo.bin"));
        assert!(context_dir.is_access_allowed("../../../tmp/a/b/foo.bin"));

        assert!(!context_dir.is_access_allowed("/file/foo.bin"));
        assert!(!context_dir.is_access_allowed("../foo.bin"));
        assert!(!context_dir.is_access_allowed("../../foo.bin"));
        assert!(!context_dir.is_access_allowed("../../file/foo.bin"));
    }

    #[test]
    fn check_filename_allowed_access_with_explicit_absolute_user_file_root() {
        // ```
        // $ cd /tmp
        // $ hurl --file-root /file test.hurl
        // ```
        let current_dir = Path::new("/tmp");
        let file_root = Path::new("/file");
        let context_dir = ContextDir::new(current_dir, file_root);
        assert!(context_dir.is_access_allowed("foo.bin")); // absolute path is /file/foo.bin
        assert!(context_dir.is_access_allowed("/file/foo.bin"));
        assert!(context_dir.is_access_allowed("a/foo.bin"));
        assert!(context_dir.is_access_allowed("a/b/foo.bin"));
        assert!(context_dir.is_access_allowed("../../file/foo.bin"));

        assert!(!context_dir.is_access_allowed("/tmp/foo.bin"));
        assert!(!context_dir.is_access_allowed("../tmp/a/b/foo.bin"));
        assert!(!context_dir.is_access_allowed("../foo.bin"));
        assert!(!context_dir.is_access_allowed("../../foo.bin"));
        assert!(!context_dir.is_access_allowed("../../../tmp/a/b/foo.bin"));

        let current_dir = Path::new("/tmp");
        let file_root = Path::new("../file");
        let context_dir = ContextDir::new(current_dir, file_root);
        assert!(context_dir.is_access_allowed("foo.bin"));
        assert!(context_dir.is_access_allowed("/file/foo.bin"));
        assert!(context_dir.is_access_allowed("a/foo.bin"));
        assert!(context_dir.is_access_allowed("a/b/foo.bin"));
        assert!(context_dir.is_access_allowed("../../file/foo.bin"));

        assert!(!context_dir.is_access_allowed("/tmp/foo.bin"));
        assert!(!context_dir.is_access_allowed("../tmp/a/b/foo.bin"));
        assert!(!context_dir.is_access_allowed("../foo.bin"));
        assert!(!context_dir.is_access_allowed("../../foo.bin"));
        assert!(!context_dir.is_access_allowed("../../../tmp/a/b/foo.bin"));
    }

    #[test]
    fn check_filename_allowed_access_with_implicit_relative_user_file_root() {
        // ```
        // $ cd /tmp
        // $ hurl a/b/test.hurl
        // ```
        let current_dir = Path::new("/tmp");
        let file_root = Path::new("a/b");
        let context_dir = ContextDir::new(current_dir, file_root);
        assert!(context_dir.is_access_allowed("foo.bin"));
        assert!(context_dir.is_access_allowed("c/foo.bin")); // absolute path is /tmp/a/b/c/foo.bin
        assert!(context_dir.is_access_allowed("/tmp/a/b/foo.bin"));
        assert!(context_dir.is_access_allowed("/tmp/a/b/c/d/foo.bin"));
        assert!(context_dir.is_access_allowed("../../../tmp/a/b/foo.bin"));

        assert!(!context_dir.is_access_allowed("/tmp/foo.bin"));
    }

    #[test]
    fn check_filename_allowed_access_with_explicit_relative_user_file_root() {
        // ```
        // $ cd /tmp
        // $ hurl --file-root ../tmp test.hurl
        // ```
        let current_dir = Path::new("/tmp");
        let file_root = Path::new("../tmp");
        let context_dir = ContextDir::new(current_dir, file_root);
        assert!(context_dir.is_access_allowed("foo.bin"));
        assert!(context_dir.is_access_allowed("/tmp/foo.bin"));
        assert!(context_dir.is_access_allowed("a/foo.bin"));
        assert!(context_dir.is_access_allowed("a/b/foo.bin"));
        assert!(context_dir.is_access_allowed("../tmp/a/b/foo.bin"));
        assert!(context_dir.is_access_allowed("../../../tmp/a/b/foo.bin"));

        assert!(!context_dir.is_access_allowed("/file/foo.bin"));
        assert!(!context_dir.is_access_allowed("../foo.bin"));
        assert!(!context_dir.is_access_allowed("../../foo.bin"));
        assert!(!context_dir.is_access_allowed("../../file/foo.bin"));
    }
}
