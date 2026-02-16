#!/bin/bash
set -Eeuo pipefail

hurl --include tests_pty/include/include.hurl
