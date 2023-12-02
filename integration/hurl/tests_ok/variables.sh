#!/bin/bash
set -Eeuo pipefail
hurl tests_ok/variables.hurl --variables-file tests_ok/variables0.properties --variables-file tests_ok/variables1.properties --variable female=true --verbose
