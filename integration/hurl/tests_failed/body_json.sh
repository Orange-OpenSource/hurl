#!/bin/bash
set -Eeuo pipefail
hurl tests_failed/body_json.hurl --variable success=invalid
