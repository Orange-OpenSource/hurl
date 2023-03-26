#!/bin/bash
set -Eeuo pipefail
hurl ssl/letsencrypt.hurl --verbose
