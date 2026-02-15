#!/bin/bash
set -Eeuo pipefail

hurl --no-color tests_failed/assert_content_encoding/assert_content_encoding.hurl
