#!/bin/bash
set -Eeuo pipefail

hurl --json --verbose tests_ok/assert/assert_header.hurl
