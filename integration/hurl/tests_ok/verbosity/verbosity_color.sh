#!/bin/bash
set -Eeuo pipefail

hurl --color --verbosity brief tests_ok/verbosity/verbosity.hurl
