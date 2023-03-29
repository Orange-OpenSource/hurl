#!/bin/bash
set -Eeuo pipefail
hurl tests_failed/assert_decompress.hurl --compressed
