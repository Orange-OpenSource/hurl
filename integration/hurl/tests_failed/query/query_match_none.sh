#!/bin/bash
set -Eeuo pipefail

hurl --no-color tests_failed/query/query_match_none.hurl
