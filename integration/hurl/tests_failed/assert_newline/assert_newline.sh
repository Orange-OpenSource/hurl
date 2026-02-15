#!/bin/bash
set -Eeuo pipefail

hurl --no-color tests_failed/assert_newline/assert_newline.hurl
