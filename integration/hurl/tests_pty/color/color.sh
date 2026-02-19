#!/bin/bash
set -Eeuo pipefail

# Default: color
hurl tests_pty/color/color.hurl

# No color
export NO_COLOR=1
hurl tests_pty/color/color.hurl
unset NO_COLOR

# Color
export HURL_COLOR=1
hurl tests_pty/color/color.hurl
unset HURL_COLOR

# No color
export HURL_COLOR=0
hurl tests_pty/color/color.hurl
unset HURL_COLOR

# No color
export HURL_NO_COLOR=1
hurl tests_pty/color/color.hurl
unset HURL_NO_COLOR

# Color
export HURL_NO_COLOR=0
hurl tests_pty/color/color.hurl
unset HURL_NO_COLOR

# No color
hurl --no-color tests_pty/color/color.hurl

# Color
hurl --color tests_pty/color/color.hurl

# Cli flags priority is greater than env vars
# No color
export HURL_NO_COLOR=0
hurl --no-color tests_pty/color/color.hurl
unset HURL_NO_COLOR
