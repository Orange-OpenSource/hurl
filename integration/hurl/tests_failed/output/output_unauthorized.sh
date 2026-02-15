#!/bin/bash
set -Eeuo pipefail

hurl --no-color tests_failed/output/output_unauthorized.hurl
