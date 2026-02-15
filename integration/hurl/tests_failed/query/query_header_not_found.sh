#!/bin/bash
set -Eeuo pipefail

hurl --no-color --json tests_failed/query/query_header_not_found.hurl
