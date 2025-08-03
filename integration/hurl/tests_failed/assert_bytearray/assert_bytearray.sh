#!/bin/bash
set -Eeuo pipefail

hurl tests_failed/assert_bytearray/assert_bytearray.hurl
