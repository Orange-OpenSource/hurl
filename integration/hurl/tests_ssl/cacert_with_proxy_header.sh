#!/bin/bash
set -Eeuo pipefail

hurl --verbosity brief \
     --proxy http://127.0.0.1:3128 \
     --proxy-header X-TO-PROXY:to-proxy \
     --header X-TO-SERVER:to-server \
     --cacert tests_ssl/certs/server/cert.pem \
     tests_ssl/cacert_with_proxy.hurl
