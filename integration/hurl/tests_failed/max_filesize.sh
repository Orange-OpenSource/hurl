#!/bin/bash
set -Eeuo pipefail
hurl --max-filesize 255 tests_failed/max_filesize.hurl
