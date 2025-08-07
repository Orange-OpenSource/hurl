#!/bin/bash
set -Eeuo pipefail

hurl tests_failed/template/template_variable_not_found.hurl
