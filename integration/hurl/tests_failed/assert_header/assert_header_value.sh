#!/bin/bash
set -Eeuo pipefail

hurl tests_failed/assert_header/assert_header_value.hurl
