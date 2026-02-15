#!/bin/bash
set -Eeuo pipefail

hurl --no-color tests_ok/verbosity/verbosity_option.hurl
