#!/bin/bash
set -Eeuo pipefail

hurl --verbose tests_ok/compressed/compressed_option.hurl
