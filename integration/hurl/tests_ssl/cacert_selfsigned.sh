#!/bin/bash
set -Eeuo pipefail
hurl tests_ssl/cacert_selfsigned.hurl --cacert tests_ssl/server/cert.selfsigned.pem --verbose
