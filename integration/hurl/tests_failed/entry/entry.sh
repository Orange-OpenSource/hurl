#!/bin/bash
set -Eeuo pipefail

hurl --no-color --from-entry 10 --to-entry 1 tests_failed/entry/entry.hurl
