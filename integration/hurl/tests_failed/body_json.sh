#!/bin/bash
set -Eeuo pipefail

hurl --variable success=invalid tests_failed/body_json.hurl
