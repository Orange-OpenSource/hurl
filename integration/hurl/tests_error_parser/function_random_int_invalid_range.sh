#!/bin/bash
set -Eeuo pipefail
hurl tests_error_parser/function_random_int_invalid_range.hurl
