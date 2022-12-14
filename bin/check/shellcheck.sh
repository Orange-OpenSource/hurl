#!/bin/bash
set -Eeuo pipefail
find . -type f -name '*.sh' -print0  | xargs -0 shellcheck

