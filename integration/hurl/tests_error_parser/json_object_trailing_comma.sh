#!/bin/bash
set -Eeuo pipefail

hurl --no-color tests_error_parser/json_object_trailing_comma.hurl
