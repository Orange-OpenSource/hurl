#!/bin/bash
set -Eeuo pipefail

export HURL_IGNORE_ASSERTS=1
hurl tests_ok/ignore_asserts/ignore_asserts.hurl
