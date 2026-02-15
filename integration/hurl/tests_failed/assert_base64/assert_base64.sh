#!/bin/bash
set -Eeuo pipefail

hurl --no-color tests_failed/assert_base64/assert_base64.hurl
