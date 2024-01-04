#!/bin/bash
set -Eeuo pipefail
hurl --json --verbose tests_ok/json_output.hurl
