#!/bin/bash
set -Eeuo pipefail

hurl --no-color tests_failed/assert_header/assert_header_not_found.hurl
