#!/bin/bash
set -Eeuo pipefail
hurl ssl/insecure.hurl --insecure --verbose
