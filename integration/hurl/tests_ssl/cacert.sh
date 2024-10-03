#!/bin/bash
set -Eeuo pipefail
hurl --cacert tests_ssl/certs/ca/cert.pem tests_ssl/cacert.hurl
