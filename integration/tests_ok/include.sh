#!/bin/bash
set -Eeuo pipefail
hurl tests_ok/include.hurl --include --verbose
