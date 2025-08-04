#!/bin/bash
set -Eeuo pipefail

hurl --compressed tests_failed/output/output_decompress.hurl
