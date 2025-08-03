#!/bin/bash
set -Eeuo pipefail

hurl tests_failed/assert_newline/assert_newline.hurl
