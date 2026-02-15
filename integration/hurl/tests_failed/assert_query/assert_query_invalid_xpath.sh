#!/bin/bash
set -Eeuo pipefail

hurl --no-color tests_failed/assert_query/assert_query_invalid_xpath.hurl
