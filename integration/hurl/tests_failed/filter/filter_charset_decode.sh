#!/bin/bash
set -Eeuo pipefail

hurl tests_failed/filter/filter_charset_decode.hurl
