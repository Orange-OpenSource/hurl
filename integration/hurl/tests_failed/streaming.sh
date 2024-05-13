#!/bin/bash
set -Eeuo pipefail
hurl --max-time 5 tests_failed/streaming.hurl


