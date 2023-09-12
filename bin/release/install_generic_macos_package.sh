#!/bin/bash
set -Eeuo pipefail

echo "----- install generic macos package -----"

# install
toolchain=$(bin/release/get_active_toolchain.sh)
echo "toolchain=${toolchain}"
generic_macos_package=$(ls target/upload/hurl-*-"${toolchain}".tar.gz)
echo "generic_macos_package=${generic_macos_package}"

install_dir="/tmp/hurl-generic-macos"
echo "install_dir=${install_dir}"
mkdir -p "${install_dir}"
tar xvf "${generic_macos_package}" -C "${install_dir}" --strip-components=1

# Return PATH var to parent shell
echo "Run this if you want to use fresh built Hurl package:"
echo "  export PATH=${install_dir}:$PATH"

