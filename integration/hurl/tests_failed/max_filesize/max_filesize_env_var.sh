#!/bin/bash
set -Eeuo pipefail

export HURL_MAX_FILESIZE=255
hurl --continue-on-error tests_failed/max_filesize/max_filesize.hurl
