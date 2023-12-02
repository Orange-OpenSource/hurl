#!/bin/bash
set -Eeuo pipefail
hurl tests_failed/query_invalid_utf8.hurl
