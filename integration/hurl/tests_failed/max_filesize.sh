#!/bin/bash
set -Eeuo pipefail
hurl --continue-on-error --max-filesize 255 tests_failed/max_filesize.hurl
