#!/bin/bash
set -Eeuo pipefail
hurl tests_failed/option_retry.hurl --verbose
