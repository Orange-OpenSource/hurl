#!/bin/bash
set -Eeuo pipefail
hurl tests_failed/query_header_not_found.hurl --json
