#!/bin/bash
set -Eeuo pipefail

hurl --no-color tests_error_parser/file_missing_delimiter.hurl
