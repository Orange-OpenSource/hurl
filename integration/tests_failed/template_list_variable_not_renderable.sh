#!/bin/bash
set -Eeuo pipefail
hurl tests_failed/template_list_variable_not_renderable.hurl
