#!/bin/bash
set -Eeuo pipefail

hurl --no-color tests_error_parser/predicate_startwith_value.hurl
