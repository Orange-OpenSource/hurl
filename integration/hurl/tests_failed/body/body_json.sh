#!/bin/bash
set -Eeuo pipefail

hurl --variable success=invalid tests_failed/body/body_json.hurl
