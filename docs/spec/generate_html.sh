#!/bin/bash
set -e
set -o pipefail

# Check version
# A specific version defines both the grammar format and the HTML output
actual_version=$(grammar --version | cut  -d" " -f2)
expected_version="0.1.0"
if [ "$actual_version" != "$expected_version" ] ; then
  echo "version mismatch"
  echo "expected: $expected_version"
  echo "actual:   $actual_version"
  exit 1
fi
cat <<END >hurl.grammar.html
<link rel="stylesheet" type="text/css" href="style.css">
END
grammar <hurl.grammar >>hurl.grammar.html