#!/bin/bash
set -Eeuo pipefail
hurl --verbose tests_ok/captures.hurl
