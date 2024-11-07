#!/bin/bash
set -Eeuo pipefail

hurl --verbose tests_ok/assert_body.hurl
