#!/bin/bash
set -Eeuo pipefail

echo "----- install generic linux package -----"

# install
generic_linux_package=$(ls target/upload/hurl-*-*.tar.gz)
echo "generic_linux_package=${generic_linux_package}"
install_dir="/tmp/hurl-generic-linux"
echo "install_dir=${install_dir}"
mkdir -p "${install_dir}"
tar xvf "${generic_linux_package}" -C "${install_dir}" --strip-components=1

# Return PATH var to parent shell
echo "Run this if you want to use fresh built Hurl package:"
echo "  export PATH=${install_dir}:$PATH"

