#!/bin/bash
set -Eeuo pipefail
bin/install_prerequisites_ubuntu.sh
bin/install_grcov.sh
bin/test/test_prerequisites.sh
bin/coverage_run.sh
lines=$(bin/coverage_uncovered_lines.py packages/hurlfmt/src/format/json.rs)
if [ -n "$lines" ]; then
    echo "$lines"
    exit 1
fi
