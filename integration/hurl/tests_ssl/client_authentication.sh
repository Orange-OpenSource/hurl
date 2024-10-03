#!/bin/bash
set -Eeuo pipefail
hurl tests_ssl/client_authentication.hurl --cacert tests_ssl/certs/server/cert.selfsigned.pem --cert tests_ssl/certs/client/cert.pem --key tests_ssl/certs/client/key.pem --verbose
