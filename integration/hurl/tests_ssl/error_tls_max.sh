#!/bin/bash
set -Eeuo pipefail
hurl tests_ssl/error_tls_max.hurl
