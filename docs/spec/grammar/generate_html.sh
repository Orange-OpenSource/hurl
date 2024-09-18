#!/bin/bash
set -Eeuo pipefail

# Check version
# A specific version defines both the grammar format and the HTML output
actual_version=$(grammar --version | cut  -d" " -f2)
expected_version="0.3.0"
if [ "$actual_version" != "$expected_version" ] ; then
  echo "version mismatch"
  echo "expected: $expected_version"
  echo "actual:   $actual_version"
  exit 1
fi


cat <<END >../../grammar.md
# Grammar

## Definitions

Short description:

- operator &#124; denotes alternative,
- operator * denotes iteration (zero or more),
- operator + denotes iteration (one or more),

## Syntax Grammar

END
grammar --section-header h3 --section-id <hurl.grammar >>../../grammar.md
