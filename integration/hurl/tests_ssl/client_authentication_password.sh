#!/bin/bash
set -Eeuo pipefail

hurl --cacert tests_ssl/certs/server/cert.selfsigned.pem \
    --cert tests_ssl/certs/client/cert.pem:foobar \
    --key tests_ssl/certs/client/encrypted.key.pem \
    tests_ssl/client_authentication_password.hurl
