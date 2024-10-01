#!/bin/bash
set -Eeuo pipefail

# Getting SSL certificates on reused connection is only supported for libcurl >= 8.2.0. We need CURLINFO_CONN_ID to
# build a map of connection id / certificates.
curl_version=$(curl --version | grep -o 'curl [0-9]\+.[0-9]\+.[0-9]\+')
if [[ "$curl_version" < "curl 8.2.0" ]]; then
    exit 255
fi

hurl tests_ssl/keepalive.hurl
