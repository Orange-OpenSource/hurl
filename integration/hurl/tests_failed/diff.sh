#!/bin/bash
set -Eeuo pipefail
hurl --continue-on-error --no-color tests_failed/diff.hurl
