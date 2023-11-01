#!/bin/bash
set -Eeuo pipefail
hurl tests_error_parser/json_object_trailing_comma.hurl
