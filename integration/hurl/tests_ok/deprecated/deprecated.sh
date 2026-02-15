#!/bin/bash
set -Eeuo pipefail

hurl --no-color tests_ok/deprecated/deprecated.hurl
