#!/bin/bash
set -Eeuo pipefail
hurl --no-color tests_failed/multiline.hurl
