#!/bin/bash
set -Eeuo pipefail

hurl --no-pretty tests_ok/assert/assert_match.hurl
