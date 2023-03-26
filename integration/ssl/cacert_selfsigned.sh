#!/bin/bash
set -Eeuo pipefail
hurl ssl/cacert_selfsigned.hurl --cacert ssl/server/cert.selfsigned.pem --verbose
