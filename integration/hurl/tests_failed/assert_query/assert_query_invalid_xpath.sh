#!/bin/bash
set -Eeuo pipefail

hurl tests_failed/assert_query/assert_query_invalid_xpath.hurl
