#!/bin/bash
set -Eeuo pipefail

hurl --no-color tests_error_parser/json_list_expecting_element.hurl
