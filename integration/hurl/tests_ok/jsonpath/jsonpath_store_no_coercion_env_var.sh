#!/bin/bash
set -Eeuo pipefail

export HURL_NO_JSONPATH_COERCION=1
hurl tests_ok/jsonpath/jsonpath_store_no_coercion.hurl
