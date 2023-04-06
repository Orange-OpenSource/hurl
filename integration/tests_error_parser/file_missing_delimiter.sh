#!/bin/bash
set -Eeuo pipefail
hurl tests_error_parser/file_missing_delimiter.hurl
