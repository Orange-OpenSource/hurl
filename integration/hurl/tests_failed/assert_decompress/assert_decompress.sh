#!/bin/bash
set -Eeuo pipefail

hurl --compressed tests_failed/assert_decompress/assert_decompress.hurl
