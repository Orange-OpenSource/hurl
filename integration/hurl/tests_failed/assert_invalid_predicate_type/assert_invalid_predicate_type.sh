#!/bin/bash
set -Eeuo pipefail

hurl --no-color tests_failed/assert_invalid_predicate_type/assert_invalid_predicate_type.hurl
