#!/bin/bash
set -Eeuo pipefail

hurl --no-color --max-time 5 tests_failed/streaming/streaming.hurl


