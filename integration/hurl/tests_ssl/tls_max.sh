#!/bin/bash
set -Eeuo pipefail
hurl --variable ver=1.2 tests_ssl/tls_max.hurl --verbose
