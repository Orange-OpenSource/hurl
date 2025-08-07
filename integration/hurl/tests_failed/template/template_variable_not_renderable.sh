#!/bin/bash
set -Eeuo pipefail

hurl --continue-on-error tests_failed/template/template_variable_not_renderable.hurl
