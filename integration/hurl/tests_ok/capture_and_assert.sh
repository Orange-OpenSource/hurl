#!/bin/bash
set -Eeuo pipefail
hurl tests_ok/capture_and_assert.hurl --verbose
