#!/bin/bash
set -Eeuo pipefail
hurl --include --color tests_ok/include.hurl
