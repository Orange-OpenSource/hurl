#!/bin/bash
set -Eeuo pipefail

hurl --continue-on-error tests_failed/file/file_unauthorized.hurl
