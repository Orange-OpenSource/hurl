#!/bin/bash
set -Eeuo pipefail

hurl --no-color tests_failed/assert_bytearray/assert_bytearray.hurl
