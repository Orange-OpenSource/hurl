#!/bin/bash
set -Eeuo pipefail
hurl tests_error_parser/invalid_url_query_param.hurl
