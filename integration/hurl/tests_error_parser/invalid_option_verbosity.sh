#!/bin/bash
set -Eeuo pipefail

hurl --no-color tests_error_parser/invalid_option_verbosity.hurl
