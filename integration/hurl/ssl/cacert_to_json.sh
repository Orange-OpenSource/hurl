#!/bin/bash
set -Eeuo pipefail
hurl --cacert ssl/ca/cert.pem --json ssl/cacert.hurl
