#!/bin/bash
set -Eeuo pipefail

hurl --no-color tests_error_parser/json_object_expecting_element.hurl
