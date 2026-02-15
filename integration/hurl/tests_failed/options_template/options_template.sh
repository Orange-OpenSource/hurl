#!/bin/bash
set -Eeuo pipefail

hurl --no-color --continue-on-error tests_failed/options_template/options_template.hurl
