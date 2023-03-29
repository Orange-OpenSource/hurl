#!/bin/bash
set -Eeuo pipefail
hurl tests_failed/query_invalid_json.hurl
