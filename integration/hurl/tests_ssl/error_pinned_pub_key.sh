#!/bin/bash
set -Eeuo pipefail

hurl --no-color \
  --cacert tests_ssl/certs/server/cert.selfsigned.pem \
  --pinnedpubkey "sha256//dGhpc2lzbm5vdGFyZWFsa2V5" \
  tests_ssl/pinned_pub_key.hurl
