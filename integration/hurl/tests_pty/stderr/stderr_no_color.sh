#!/bin/bash
set -Eeuo pipefail

hurl --no-color tests_pty/stderr/stderr.hurl
