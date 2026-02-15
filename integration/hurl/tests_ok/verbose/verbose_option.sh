#!/bin/bash
set -Eeuo pipefail

hurl --no-color tests_ok/verbose/verbose_option.hurl
