#!/bin/bash
set -Eeuo pipefail
xmllint --noout integration/tests_*/*.html
