#!/bin/bash
set -Eeuo pipefail

hurl --no-color --variable success=invalid tests_failed/body/body_json.hurl
