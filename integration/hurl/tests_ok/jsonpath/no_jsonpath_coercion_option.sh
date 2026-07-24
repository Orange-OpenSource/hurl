#!/bin/bash
set -Eeuo pipefail

hurl tests_ok/jsonpath/no_jsonpath_coercion_option.hurl
