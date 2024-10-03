#!/bin/bash
set -Eeuo pipefail
hurl --cacert tests_ssl/certs/ca/cert.pem --json tests_ssl/cacert.hurl
