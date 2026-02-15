#!/bin/bash
set -Eeuo pipefail

hurl --no-color --very-verbose --location tests_ok/very_verbose/very_verbose.hurl
