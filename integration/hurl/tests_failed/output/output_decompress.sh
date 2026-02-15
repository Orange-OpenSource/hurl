#!/bin/bash
set -Eeuo pipefail

hurl --no-color --compressed tests_failed/output/output_decompress.hurl
