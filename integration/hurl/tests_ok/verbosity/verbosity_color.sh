#!/bin/bash
set -Eeuo pipefail

hurl --verbosity brief tests_ok/verbosity/verbosity.hurl
