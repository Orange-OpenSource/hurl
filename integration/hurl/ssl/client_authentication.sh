#!/bin/bash
set -Eeuo pipefail
hurl ssl/client_authentication.hurl --cacert ssl/server/cert.selfsigned.pem --cert ssl/client/cert.pem --key ssl/client/key.pem --verbose
