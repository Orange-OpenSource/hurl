#!/bin/bash
set -Eeuo pipefail

hurl tests_failed/query/query_invalid_json.hurl
