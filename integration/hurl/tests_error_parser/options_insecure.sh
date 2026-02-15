#!/bin/bash
set -Eeuo pipefail

hurl --no-color tests_error_parser/options_insecure.hurl
