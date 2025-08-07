#!/bin/bash
set -Eeuo pipefail

hurl --verbose tests_failed/retry/retry_option.hurl
