#!/bin/bash
set -Eeuo pipefail

hurl --no-color tests_failed/assert_file/assert_file.hurl
