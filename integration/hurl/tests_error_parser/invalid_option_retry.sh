#!/bin/bash
set -Eeuo pipefail

hurl tests_error_parser/invalid_option_retry.hurl
