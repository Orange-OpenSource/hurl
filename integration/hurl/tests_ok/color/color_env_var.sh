#!/bin/bash
set -Eeuo pipefail

export HURL_COLOR=1
hurl --include tests_ok/color/color.hurl
