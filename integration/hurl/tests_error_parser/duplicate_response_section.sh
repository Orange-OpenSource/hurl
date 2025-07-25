#!/bin/bash
set -Eeuo pipefail
hurl tests_error_parser/duplicate_response_section.hurl
