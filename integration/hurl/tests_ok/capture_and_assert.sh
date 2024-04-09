#!/bin/bash
set -Eeuo pipefail
hurl --verbose tests_ok/capture_and_assert.hurl
