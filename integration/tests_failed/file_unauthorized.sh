#!/bin/bash
set -Eeuo pipefail
hurl tests_failed/file_unauthorized.hurl --continue-on-error
