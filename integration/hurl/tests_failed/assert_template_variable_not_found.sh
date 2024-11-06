#!/bin/bash
set -Eeuo pipefail

hurl --json tests_failed/assert_template_variable_not_found.hurl
