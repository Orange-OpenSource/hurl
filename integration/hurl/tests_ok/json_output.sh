#!/bin/bash
set -Eeuo pipefail
hurl tests_ok/json_output.hurl --json --verbose
