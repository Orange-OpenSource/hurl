#!/bin/bash
set -Eeuo pipefail


# The tests for certificate reuse need a libcurl >= 8.2.0
skip_cert_test=true
curl_version=$(curl --version | grep -o '^curl [0-9.]*' | head -n1 | cut -d' ' -f2)
IFS=. read -r major minor _ <<< "$curl_version"
if (( major > 8 || (major == 8 && minor >= 2) )); then
    skip_cert_test=false
fi

hurl --variable skip_cert_test=$skip_cert_test tests_ssl/live.hurl
