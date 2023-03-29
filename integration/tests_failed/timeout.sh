#!/bin/bash
set -Eeuo pipefail
hurl tests_failed/timeout.hurl --max-time 1
