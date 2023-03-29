#!/bin/bash
set -Eeuo pipefail
hurl tests_failed/assert_bytearray.hurl
