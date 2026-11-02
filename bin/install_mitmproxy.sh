#!/bin/bash
set -Eeuo pipefail

# Install mitmproxy standalone binaries (mitmproxy, mitmdump, mitmweb) on glibc-based Linux.
# Alpine (musl) can't run these binaries and uses its own mitmproxy package instead.
# See <https://docs.mitmproxy.org/stable/overview/installation/>

version="${MITMPROXY_VERSION:-12.3.1}"
install_dir="${MITMPROXY_INSTALL_DIR:-/usr/local/bin}"

case "$(uname -m)" in
    x86_64|amd64) arch="x86_64" ;;
    aarch64|arm64) arch="aarch64" ;;
    *) echo "Unsupported architecture $(uname -m)" ; exit 1 ;;
esac

echo "----- install mitmproxy ${version} (${arch}) -----"
url="https://downloads.mitmproxy.org/${version}/mitmproxy-${version}-linux-${arch}.tar.gz"
tmp_dir=$(mktemp -d)
curl --fail --silent --show-error --location "${url}" --output "${tmp_dir}/mitmproxy.tar.gz"
tar -xzf "${tmp_dir}/mitmproxy.tar.gz" -C "${tmp_dir}"
sudo install -m 755 "${tmp_dir}/mitmproxy" "${tmp_dir}/mitmdump" "${tmp_dir}/mitmweb" "${install_dir}"
rm -rf "${tmp_dir}"
mitmdump --version
