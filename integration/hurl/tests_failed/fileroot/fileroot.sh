#!/bin/bash
set -Eeuo pipefail

# We test that a symlink cannot access outside the file-root (directory containing the Hurl file by default)
# Symlinks can be a file, or a directory containing a file that doesn't exist yet.

rm -rf /tmp/unauthorized_dir
mkdir -p /tmp/unauthorized_dir
touch /tmp/unauthorized.bin
ln -fs /tmp/unauthorized.bin tests_failed/fileroot/authorized.bin
ln -fns /tmp/unauthorized_dir tests_failed/fileroot/authorized_dir
hurl --continue-on-error tests_failed/fileroot/fileroot.hurl
