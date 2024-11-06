#!/bin/bash
set -Eeuo pipefail

hurl --json --verbose tests_ok/assert_header.hurl
