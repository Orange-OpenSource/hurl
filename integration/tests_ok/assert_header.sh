#!/bin/bash
set -Eeuo pipefail
hurl tests_ok/assert_header.hurl --json --verbose
