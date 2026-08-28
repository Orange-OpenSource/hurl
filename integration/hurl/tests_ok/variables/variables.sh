#!/bin/bash
set -Eeuo pipefail

XDG_CONFIG_HOME=$(dirname "$0")/config
export XDG_CONFIG_HOME

hurl --variables-file tests_ok/variables/variables0.env \
     --variables-file tests_ok/variables/variables1.env \
     --variable female=true \
     tests_ok/variables/variables.hurl
