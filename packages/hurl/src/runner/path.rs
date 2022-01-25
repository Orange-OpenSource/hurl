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
use std::path::{Component, Path, PathBuf};

/// Return true if `path` is a descendant path of ancestor, false otherwise
pub fn is_descendant(path: &Path, ancestor: &Path) -> bool {
    let path = normalize_path(path);
    let ancestor = normalize_path(ancestor);
    for a in path.ancestors() {
        if ancestor == a {
            return true;
        }
    }
    false
}

/// Returns the absolute form of the path with all intermediate components normalized.
/// Contrary to the methods `canonicalize` on `Path`, this function doesn't require
/// the final path to exist.
/// Borrowed from https://github.com/rust-lang/cargo src/cargo/util/paths.rs
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

#[cfg(test)]
mod tests {
    use super::*;

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
