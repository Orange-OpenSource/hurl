#!/bin/bash
set -e

# Check version
EXPECTED="1.48.0"
ACTUAL="$(cargo --version | cut -d" " -f2)"
echo "check that version is $EXPECTED"
if [ "$EXPECTED" != "$ACTUAL" ]; then
    echo "Failure! version is $ACTUAL"
    exit 1
fi

