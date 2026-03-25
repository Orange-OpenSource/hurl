#!/bin/bash
set -Eeuo pipefail

export HURL_NO_ASSERT=1
hurl tests_ok/ignore_asserts/ignore_asserts.hurl
