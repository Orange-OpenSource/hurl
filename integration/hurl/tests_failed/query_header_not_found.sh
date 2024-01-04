#!/bin/bash
set -Eeuo pipefail
hurl --json tests_failed/query_header_not_found.hurl
