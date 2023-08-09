#!/bin/bash
set -Eeuo pipefail
hurl --http1.0 tests_ok/http_version.hurl
