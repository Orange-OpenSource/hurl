#!/bin/bash
set -Eeuo pipefail
hurl ssl/cacert.hurl --cacert ssl/ca/cert.pem --verbose
