#!/bin/bash
set -Eeuo pipefail

hurl --no-color --continue-on-error --max-filesize 255 tests_failed/max_filesize/max_filesize.hurl
