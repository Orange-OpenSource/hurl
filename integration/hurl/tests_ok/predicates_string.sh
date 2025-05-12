#!/bin/bash
set -Eeuo pipefail

hurl --verbose tests_ok/predicates_string.hurl
