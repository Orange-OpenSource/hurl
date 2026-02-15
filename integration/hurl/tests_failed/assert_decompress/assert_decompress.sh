#!/bin/bash
set -Eeuo pipefail

hurl --no-color --compressed tests_failed/assert_decompress/assert_decompress.hurl
