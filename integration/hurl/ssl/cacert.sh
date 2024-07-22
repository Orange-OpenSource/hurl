#!/bin/bash
set -Eeuo pipefail
hurl --cacert ssl/ca/cert.pem ssl/cacert.hurl
