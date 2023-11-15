#!/bin/bash
set -Eeuo pipefail
hurl --continue-on-error tests_failed/template_variables_not_renderable.hurl
