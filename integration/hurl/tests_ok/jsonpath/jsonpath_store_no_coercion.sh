#!/bin/bash
set -Eeuo pipefail

hurl --no-jsonpath-coercion tests_ok/jsonpath/jsonpath_store_no_coercion.hurl
