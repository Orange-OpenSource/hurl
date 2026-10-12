#!/bin/bash
set -Eeuo pipefail

hurl --cacert tests_ssl/certs/server/cert.selfsigned.pem tests_ssl/cacert_selfsigned.hurl
