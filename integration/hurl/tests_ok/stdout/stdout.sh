#!/bin/bash
set -Eeuo pipefail

hurl --no-color --verbose --output - tests_ok/stdout/stdout.hurl


