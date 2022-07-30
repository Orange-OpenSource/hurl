#!/bin/sh
set -e
set -u

os="$1"
package_dir="target/tarball/hurl-$VERSION"
tarball_file="hurl-$VERSION-x86_64-$os.tar.gz"

mkdir -p "$package_dir"
cp target/release/hurl "$package_dir"
cp target/release/hurlfmt "$package_dir"
cp target/man/* "$package_dir"

mkdir -p target/upload
tar cvfz "target/upload/$tarball_file" -C "$(dirname "$package_dir")" "hurl-$VERSION"

