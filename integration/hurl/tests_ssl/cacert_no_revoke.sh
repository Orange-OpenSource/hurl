#!/bin/bash
set -Eeuo pipefail
# --ssl-no-revoke is only available in windows
#hurl --cacert tests_ssl/certs/ca/cert.pem --ssl-no-revoke tests_ssl/cacert.hurl
exit 255
