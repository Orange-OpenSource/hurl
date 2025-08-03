#!/bin/bash
set -Eeuo pipefail

hurl --json tests_failed/assert_match_utf8/assert_match_utf8.hurl
