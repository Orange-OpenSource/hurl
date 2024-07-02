#!/bin/bash
set -Eeuo pipefail

hurl --no-output tests_ok/query_cache.hurl
