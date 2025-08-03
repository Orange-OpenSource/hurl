#!/bin/bash
set -Eeuo pipefail

hurl tests_failed/assert_content_encoding/assert_content_encoding.hurl
