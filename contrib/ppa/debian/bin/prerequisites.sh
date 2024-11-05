#!/bin/bash
set -Eeuo pipefail

echo "## untar pxf vendor.tar.xz:"
tar pxf vendor.tar.xz

echo "## install rustc and cargo"
rust_version=$(grep '^rust-version' packages/hurl/Cargo.toml | cut -f2 -d'"')
arch=$(uname -m)
package="rust-${rust_version}-${arch}-unknown-linux-gnu"
packagelight="${package}-light"
echo "rust_version=${rust_version}"
echo "architecture=${arch}"
echo "packagelight=${packagelight}"
tar xf "${packagelight}.tar.xz"
mkdir -p /tmp/rust
./"${packagelight}"/install.sh --verbose --destdir=/tmp/rust --disable-ldconfig
export PATH="/tmp/rust/usr/local/bin:$PATH"
which rustc
which cargo
rustc --version
cargo --version

echo "## .cargo/config:"
cat .cargo/config

echo "## update release.sh"
sed -i "s#\"\$HOME\"/.cargo/bin#/tmp/rust/usr/local/bin#g" bin/release/release.sh
cat bin/release/release.sh
