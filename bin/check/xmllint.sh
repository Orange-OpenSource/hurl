#!/bin/bash
set -Eeuo pipefail

xmllint --noout integration/hurlfmt/tests_*/*.html

