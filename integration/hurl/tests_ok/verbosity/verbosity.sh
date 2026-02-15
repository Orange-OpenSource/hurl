#!/bin/bash
set -Eeuo pipefail

hurl --no-color --verbosity brief tests_ok/verbosity/verbosity.hurl
