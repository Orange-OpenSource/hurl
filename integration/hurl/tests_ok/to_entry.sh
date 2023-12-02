#!/bin/bash
set -Eeuo pipefail
hurl tests_ok/to_entry.hurl --to-entry 3 --verbose
