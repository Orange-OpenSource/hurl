#!/bin/bash
set -Eeuo pipefail
hurl tests_failed/template_obj_variable_not_renderable.hurl
