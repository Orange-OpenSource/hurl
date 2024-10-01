#!/bin/bash
set -Eeuo pipefail
hurl --cacert tests_ssl/ca/cert.pem --json tests_ssl/cacert.hurl
