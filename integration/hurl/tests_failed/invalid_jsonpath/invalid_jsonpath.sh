#!/bin/bash
set -Eeuo pipefail

hurl --no-color tests_failed/invalid_jsonpath/invalid_jsonpath.hurl
