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
//! Access controlled path.
use std::path::{Component, Path, PathBuf};

/// Represents the directories used to run a Hurl file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContextDir {
    /// The current working directory.
    /// If current directory is a relative path, the `is_access_allowed` method
    /// is not guaranteed to be correct.
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
    pub fn resolved_path(&self, filename: &Path) -> PathBuf {
        self.file_root.join(filename)
    }

    /// Checks if a given `filename` access is authorized.
    /// This method is used to check if a local file can be included in POST request or if a
    /// response can be outputted to a given file when using `output` option in \[Options\] sections.
    pub fn is_access_allowed(&self, filename: &Path) -> bool {
        let file = self.resolved_path(filename);
        let absolute_file = self.current_dir.join(file);
        let absolute_file_root = self.current_dir.join(&self.file_root);
        is_descendant(absolute_file.as_path(), absolute_file_root.as_path())
    }
}

/// Return true if `path` is a descendant path of `ancestor`, false otherwise.
fn is_descendant(path: &Path, ancestor: &Path) -> bool {
    let path = normalize_path(path);
    let ancestor = normalize_path(ancestor);
    for a in path.ancestors() {
        if ancestor == a {
            return true;
        }
    }
    false
}

/// Returns the absolute form of this `path` with all intermediate components normalized.
/// Contrary to the methods [`std::fs::canonicalize`] on [`Path`], this function doesn't require
/// the final path to exist.
///
/// Borrowed from https://github.com/rust-lang/cargo/blob/master/crates/cargo-util/src/paths.rs
fn normalize_path(path: &Path) -> PathBuf {
    let mut components = path.components().peekable();
    let mut ret = if let Some(c @ Component::Prefix(..)) = components.peek().cloned() {
        components.next();
        PathBuf::from(c.as_os_str())
    } else {
        PathBuf::new()
    };

    for component in components {
        match component {
            Component::Prefix(..) => unreachable!(),
            Component::RootDir => {
                ret.push(component.as_os_str());
            }
            Component::CurDir => {}
            Component::ParentDir => {
                ret.pop();
            }
            Component::Normal(c) => {
                ret.push(c);
            }
        }
    }
    ret
}

// Create parent directories, if missing, given a filepath ending with a file name
pub fn create_dir_all(filename: &Path) -> Result<(), std::io::Error> {
    if let Some(parent) = filename.parent() {
        return std::fs::create_dir_all(parent);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_filename_allowed_access_without_user_file_root() {
        // ```
        // $ cd /tmp
        // $ hurl test.hurl
        // ```
        let current_dir = Path::new("/tmp");
        let file_root = Path::new("");
        let ctx = ContextDir::new(current_dir, file_root);
        assert!(ctx.is_access_allowed(Path::new("foo.bin")));
        assert!(ctx.is_access_allowed(Path::new("/tmp/foo.bin")));
        assert!(ctx.is_access_allowed(Path::new("a/foo.bin")));
        assert!(ctx.is_access_allowed(Path::new("a/b/foo.bin")));
        assert!(ctx.is_access_allowed(Path::new("../tmp/a/b/foo.bin")));
        assert!(ctx.is_access_allowed(Path::new("../../../tmp/a/b/foo.bin")));

        assert!(!ctx.is_access_allowed(Path::new("/file/foo.bin")));
        assert!(!ctx.is_access_allowed(Path::new("../foo.bin")));
        assert!(!ctx.is_access_allowed(Path::new("../../foo.bin")));
        assert!(!ctx.is_access_allowed(Path::new("../../file/foo.bin")));
    }

    #[test]
    fn check_filename_allowed_access_with_explicit_absolute_user_file_root() {
        // ```
        // $ cd /tmp
        // $ hurl --file-root /file test.hurl
        // ```
        let current_dir = Path::new("/tmp");
        let file_root = Path::new("/file");
        let ctx = ContextDir::new(current_dir, file_root);
        assert!(ctx.is_access_allowed(Path::new("foo.bin"))); // absolute path is /file/foo.bin
        assert!(ctx.is_access_allowed(Path::new("/file/foo.bin")));
        assert!(ctx.is_access_allowed(Path::new("a/foo.bin")));
        assert!(ctx.is_access_allowed(Path::new("a/b/foo.bin")));
        assert!(ctx.is_access_allowed(Path::new("../../file/foo.bin")));

        assert!(!ctx.is_access_allowed(Path::new("/tmp/foo.bin")));
        assert!(!ctx.is_access_allowed(Path::new("../tmp/a/b/foo.bin")));
        assert!(!ctx.is_access_allowed(Path::new("../foo.bin")));
        assert!(!ctx.is_access_allowed(Path::new("../../foo.bin")));
        assert!(!ctx.is_access_allowed(Path::new("../../../tmp/a/b/foo.bin")));

        let current_dir = Path::new("/tmp");
        let file_root = Path::new("../file");
        let ctx = ContextDir::new(current_dir, file_root);
        assert!(ctx.is_access_allowed(Path::new("foo.bin")));
        assert!(ctx.is_access_allowed(Path::new("/file/foo.bin")));
        assert!(ctx.is_access_allowed(Path::new("a/foo.bin")));
        assert!(ctx.is_access_allowed(Path::new("a/b/foo.bin")));
        assert!(ctx.is_access_allowed(Path::new("../../file/foo.bin")));

        assert!(!ctx.is_access_allowed(Path::new("/tmp/foo.bin")));
        assert!(!ctx.is_access_allowed(Path::new("../tmp/a/b/foo.bin")));
        assert!(!ctx.is_access_allowed(Path::new("../foo.bin")));
        assert!(!ctx.is_access_allowed(Path::new("../../foo.bin")));
        assert!(!ctx.is_access_allowed(Path::new("../../../tmp/a/b/foo.bin")));
    }

    #[test]
    fn check_filename_allowed_access_with_implicit_relative_user_file_root() {
        // ```
        // $ cd /tmp
        // $ hurl a/b/test.hurl
        // ```
        let current_dir = Path::new("/tmp");
        let file_root = Path::new("a/b");
        let ctx = ContextDir::new(current_dir, file_root);
        assert!(ctx.is_access_allowed(Path::new("foo.bin")));
        assert!(ctx.is_access_allowed(Path::new("c/foo.bin"))); // absolute path is /tmp/a/b/c/foo.bin
        assert!(ctx.is_access_allowed(Path::new("/tmp/a/b/foo.bin")));
        assert!(ctx.is_access_allowed(Path::new("/tmp/a/b/c/d/foo.bin")));
        assert!(ctx.is_access_allowed(Path::new("../../../tmp/a/b/foo.bin")));

        assert!(!ctx.is_access_allowed(Path::new("/tmp/foo.bin")));
    }

    #[test]
    fn check_filename_allowed_access_with_explicit_relative_user_file_root() {
        // ```
        // $ cd /tmp
        // $ hurl --file-root ../tmp test.hurl
        // ```
        let current_dir = Path::new("/tmp");
        let file_root = Path::new("../tmp");
        let ctx = ContextDir::new(current_dir, file_root);
        assert!(ctx.is_access_allowed(Path::new("foo.bin")));
        assert!(ctx.is_access_allowed(Path::new("/tmp/foo.bin")));
        assert!(ctx.is_access_allowed(Path::new("a/foo.bin")));
        assert!(ctx.is_access_allowed(Path::new("a/b/foo.bin")));
        assert!(ctx.is_access_allowed(Path::new("../tmp/a/b/foo.bin")));
        assert!(ctx.is_access_allowed(Path::new("../../../tmp/a/b/foo.bin")));

        assert!(!ctx.is_access_allowed(Path::new("/file/foo.bin")));
        assert!(!ctx.is_access_allowed(Path::new("../foo.bin")));
        assert!(!ctx.is_access_allowed(Path::new("../../foo.bin")));
        assert!(!ctx.is_access_allowed(Path::new("../../file/foo.bin")));
    }

    #[test]
    fn is_descendant_true() {
        let child = Path::new("/tmp/foo/bar.txt");
        let parent = Path::new("/tmp");
        assert!(is_descendant(child, parent));

        let child = Path::new("/tmp/foo/../bar.txt");
        let parent = Path::new("/tmp");
        assert!(is_descendant(child, parent));

        let child = Path::new("bar.txt");
        let parent = Path::new("");
        assert!(is_descendant(child, parent));
    }

    #[test]
    fn is_descendant_false() {
        let child = Path::new("/tmp/foo/../../bar.txt");
        let parent = Path::new("/tmp");
        assert!(!is_descendant(child, parent));

        let child = Path::new("/a/bar.txt");
        let parent = Path::new("/b");
        assert!(!is_descendant(child, parent));

        let child = Path::new("/bar.txt");
        let parent = Path::new("");
        assert!(!is_descendant(child, parent));
    }
}
