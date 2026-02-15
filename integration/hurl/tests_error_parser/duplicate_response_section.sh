#!/bin/bash
set -Eeuo pipefail

hurl --no-color tests_error_parser/duplicate_response_section.hurl
