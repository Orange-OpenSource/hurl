#!/bin/bash
set -Eeuo pipefail
hurl tests_ok/post_base64.hurl --verbose
