#!/bin/bash
set -Eeuo pipefail

hurl tests_failed/invalid_jsonpath/invalid_jsonpath.hurl
