#!/bin/bash
set -Eeuo pipefail
hurl --verbose tests_ok/form_params.hurl
