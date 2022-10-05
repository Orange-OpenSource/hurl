#!/bin/sh
set -e

echo "----- install generic linux package -----"

# install
generic_linux_package=$(ls target/upload/hurl-*-x86_64-linux.tar.gz)
install_dir="/tmp/hurl-generic-linux"
mkdir -p "${install_dir}"
tar xvf "${generic_linux_package}" -C "${install_dir}" --strip-components=1

# Return PATH var to parent shell
echo "Run this if you want to use fresh builded hurl package:"
echo "  export PATH=${install_dir}:$PATH"
