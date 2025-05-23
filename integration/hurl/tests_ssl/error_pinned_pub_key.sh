#!/bin/bash
set -Eeuo pipefail
hurl tests_ssl/pinned_pub_key.hurl --cacert tests_ssl/certs/server/cert.selfsigned.pem --pinnedpubkey "sha256//dGhpc2lzbm5vdGFyZWFsa2V5"
