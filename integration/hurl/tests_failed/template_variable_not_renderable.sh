#!/bin/bash
set -Eeuo pipefail
hurl tests_failed/template_variable_not_renderable.hurl --continue-on-error
