#!/bin/bash
set -Eeuo pipefail
hurl tests_error_parser/url_bad_prefix.hurl
