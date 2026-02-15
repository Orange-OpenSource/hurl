#!/bin/bash
set -Eeuo pipefail

hurl --include tests_ok/include/include.hurl
