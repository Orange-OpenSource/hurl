#!/bin/bash
set -Eeuo pipefail

hurl --no-color --verbose tests_ok/verbose/verbose.hurl
