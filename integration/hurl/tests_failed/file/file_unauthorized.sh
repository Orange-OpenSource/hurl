#!/bin/bash
set -Eeuo pipefail

hurl --no-color --continue-on-error tests_failed/file/file_unauthorized.hurl
