#!/bin/bash
set -Eeuo pipefail

hurl --no-color tests_error_parser/typo_option.hurl
