#!/bin/bash
set -Eeuo pipefail

hurl tests_failed/template_variable_not_found.hurl
