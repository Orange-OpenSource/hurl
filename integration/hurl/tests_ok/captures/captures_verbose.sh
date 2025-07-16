#!/bin/bash
set -Eeuo pipefail

hurl --verbose --color tests_ok/captures/captures_verbose.hurl
