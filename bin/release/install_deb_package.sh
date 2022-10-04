#!/bin/sh
set -e

echo "----- install deb package -----"

# install
package_dir="$(cd target/upload ; pwd)"
deb_package=$(ls "${package_dir}"/hurl_*_amd64.deb)
install_dir="/tmp/hurl-deb-package"
mkdir -p "${install_dir}"
dpkg -x "${deb_package}" "${install_dir}"

# Return PATH var to parent shell
package_dir="${install_dir}/usr/bin"
echo "Run this if you want to use fresh builded hurl package:"
echo "  export PATH=$package_dir:$PATH"
