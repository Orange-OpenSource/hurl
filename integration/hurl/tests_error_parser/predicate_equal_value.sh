#!/bin/bash
set -Eeuo pipefail

hurl --no-color tests_error_parser/predicate_equal_value.hurl
