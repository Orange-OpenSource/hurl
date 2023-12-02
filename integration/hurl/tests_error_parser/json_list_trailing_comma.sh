#!/bin/bash
set -Eeuo pipefail
hurl tests_error_parser/json_list_trailing_comma.hurl
