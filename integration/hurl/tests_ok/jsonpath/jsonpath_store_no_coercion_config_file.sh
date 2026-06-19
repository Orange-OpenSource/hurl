#!/bin/bash
set -Eeuo pipefail

XDG_CONFIG_HOME=$(dirname "$0")/config_jsonpath_no_coercion
export XDG_CONFIG_HOME

hurl tests_ok/jsonpath/jsonpath_store_no_coercion.hurl
