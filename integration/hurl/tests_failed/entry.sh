#!/bin/bash
set -Eeuo pipefail
hurl --from-entry 10 --to-entry 1 tests_failed/entry.hurl
