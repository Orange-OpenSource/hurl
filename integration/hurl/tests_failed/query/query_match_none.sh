#!/bin/bash
set -Eeuo pipefail

hurl tests_failed/query/query_match_none.hurl
