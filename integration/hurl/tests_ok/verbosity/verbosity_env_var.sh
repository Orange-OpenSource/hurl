#!/bin/bash
set -Eeuo pipefail

export HURL_VERBOSE=0
hurl tests_ok/verbosity/verbosity.hurl
unset HURL_VERBOSE

export HURL_VERBOSE=1
hurl tests_ok/verbosity/verbosity.hurl
unset HURL_VERBOSE

export HURL_VERY_VERBOSE=1
hurl tests_ok/verbosity/verbosity.hurl
unset HURL_VERY_VERBOSE

export HURL_VERBOSITY=brief
hurl tests_ok/verbosity/verbosity.hurl
unset HURL_VERBOSITY

# Overrides env var

export HURL_VERBOSITY=brief
hurl --verbose tests_ok/verbosity/verbosity.hurl
unset HURL_VERBOSITY

