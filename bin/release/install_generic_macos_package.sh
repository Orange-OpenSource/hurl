#!/bin/bash
set -Eeuo pipefail

echo "----- install generic macos package -----"

# install
os=$(uname -a | cut -d ' ' -f 1 | tr "[:upper:]" "[:lower:]")
echo "os=${os}"
arch="$(uname -m)"
echo "arch=${arch}"
generic_macos_package=$(ls target/upload/hurl-*-"${arch}"-"${os}".tar.gz)

install_dir="/tmp/hurl-generic-macos"
mkdir -p "${install_dir}"
tar xvf "${generic_macos_package}" -C "${install_dir}" --strip-components=1

# Return PATH var to parent shell
echo "Run this if you want to use fresh built Hurl package:"
echo "  export PATH=${install_dir}:$PATH"

