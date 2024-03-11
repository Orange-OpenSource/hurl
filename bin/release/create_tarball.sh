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
mkdir -p "${package_dir}/bin"
mkdir -p "${package_dir}/completions"
mkdir -p "${package_dir}/man/man1"

cp target/release/{hurl,hurlfmt} "${package_dir}/bin"
cp completions/{_hurl,_hurlfmt,hurl.bash,hurlfmt.bash,hurl.fish,hurlfmt.fish} "${package_dir}/completions"
cp target/man/* "${package_dir}/man/man1"

mkdir -p target/upload
tar cvfz "target/upload/${tarball_file}" -C "$(dirname "${package_dir}")" "hurl-${package_signature}"

