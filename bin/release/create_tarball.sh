#!/bin/bash
set -Eeuo pipefail

toolchain=$(bin/release/get_active_toolchain.sh)
echo "toolchain=${toolchain}"
package_signature="${VERSION}-${toolchain}"
echo "package_signature=${package_signature=}"
package_dir="target/tarball/hurl-${package_signature}"
echo "package_dir=${package_dir}"
tarball_file="hurl-${package_signature}.tar.gz"
echo "tarball_file=${tarball_file}"

mkdir -p "${package_dir}"
cp target/release/hurl "${package_dir}"
cp target/release/hurlfmt "${package_dir}"
cp target/man/* "${package_dir}"

mkdir -p target/upload
tar cvfz "target/upload/${tarball_file}" -C "$(dirname "${package_dir}")" "hurl-${package_signature}"

