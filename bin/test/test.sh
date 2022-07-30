#!/bin/sh
set -e
"$(dirname "$0")"/test_prerequisites.sh
"$(dirname "$0")"/test_unit.sh
"$(dirname "$0")"/test_integ.sh
