#!/bin/bash
set -e

"$(dirname "$0")"/test_prerequisites.sh
"$(dirname "$0")"/test_unit.sh
export PATH="${PWD}/target/release:$PATH"
"$(dirname "$0")"/test_integ.sh
