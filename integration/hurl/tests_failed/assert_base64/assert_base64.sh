#!/bin/bash
set -Eeuo pipefail

hurl tests_failed/assert_base64/assert_base64.hurl
