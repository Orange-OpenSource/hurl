#!/bin/bash
set -Eeuo pipefail

hurl --verbosity brief \
     --proxy http://127.0.0.1:3128 \
     --cacert tests_ssl/certs/server/cert.pem \
     --json \
     tests_ssl/cacert_with_proxy.hurl \
     | jq '.entries[0].calls[0].request.headers'
