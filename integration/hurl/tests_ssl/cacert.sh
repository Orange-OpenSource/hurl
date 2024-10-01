#!/bin/bash
set -Eeuo pipefail
hurl --cacert tests_ssl/ca/cert.pem tests_ssl/cacert.hurl
