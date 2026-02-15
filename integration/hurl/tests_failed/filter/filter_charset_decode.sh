#!/bin/bash
set -Eeuo pipefail

hurl --no-color tests_failed/filter/filter_charset_decode.hurl
