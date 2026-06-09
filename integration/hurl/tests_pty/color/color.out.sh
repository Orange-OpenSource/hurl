#!/bin/bash
# Generate expected color.out
# AD_HOC_IGNORE_check_integration_sh_ps1_consistency
set -Eeuo pipefail

cd "$(dirname "$0")"
{
hurl color.hurl --pretty --color
hurl color.hurl --pretty --no-color
hurl color.hurl --pretty --color
hurl color.hurl --pretty --no-color
hurl color.hurl --pretty --color
hurl color.hurl --pretty --no-color
hurl color.hurl --pretty --no-color
hurl color.hurl --pretty --color
hurl color.hurl --pretty --no-color
hurl color.hurl --pretty --color
hurl color.hurl --pretty --no-color
} > color.out

