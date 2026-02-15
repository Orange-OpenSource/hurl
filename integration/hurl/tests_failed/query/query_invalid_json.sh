#!/bin/bash
set -Eeuo pipefail

hurl --no-color tests_failed/query/query_invalid_json.hurl
