#!/bin/bash
set -Eeuo pipefail

hurl --no-color --include tests_ok/include/include.hurl
