#!/bin/bash
set -Eeuo pipefail
hurl tests_failed/output_unauthorized.hurl
