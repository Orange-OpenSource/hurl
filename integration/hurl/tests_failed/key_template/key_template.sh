#!/bin/bash
set -Eeuo pipefail

hurl --no-color tests_failed/key_template/key_template.hurl
