#!/bin/bash
set -Eeuo pipefail

hurl --no-color tests_pty/binary_output/binary_output.hurl
