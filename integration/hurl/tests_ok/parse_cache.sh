#!/bin/bash
set -Eeuo pipefail

hurl --no-output tests_ok/parse_cache.hurl
