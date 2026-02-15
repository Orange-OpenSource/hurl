#!/bin/bash
set -Eeuo pipefail

hurl --no-color tests_failed/template/template_variable_not_found.hurl
