#!/bin/bash
set -Eeuo pipefail

hurl --no-color --pretty tests_ok/pretty/pretty.hurl
