#!/bin/bash
set -Eeuo pipefail

hurl --verbose tests_ok/assert/assert_body.hurl
