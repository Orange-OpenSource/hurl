#!/bin/bash
set -Eeuo pipefail

set +eo pipefail
hurl --version | grep Features | grep -q NTLM
if [ $? -eq 1 ]; then
  exit 255
fi
# On Fedora, MD4 is not available by default in OpenSSL, NTLM handshake loops until max redirects
if grep -qsE '^ID="?fedora"?$' /etc/os-release; then
  exit 255
fi
set -Eeuo pipefail

hurl --ntlm -u ":" tests_ok/ntlm/ntlm.hurl
