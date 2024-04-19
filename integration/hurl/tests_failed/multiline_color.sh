#!/bin/bash
set -Eeuo pipefail
hurl --color tests_failed/multiline.hurl
