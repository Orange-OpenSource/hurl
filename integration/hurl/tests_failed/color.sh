#!/bin/bash
set -Eeuo pipefail
hurl --continue-on-error --color tests_failed/color.hurl
