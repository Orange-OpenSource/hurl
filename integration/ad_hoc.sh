#!/bin/bash
# add more tests
# that can be easily added in tests_ok/ and tests_failed/

echo "Check file not found error"
actual=$(hurl does_not_exist.hurl 2>&1)
expected="error: hurl: cannot access 'does_not_exist.hurl': No such file or directory"
if [ "$actual" != "$expected" ]; then
    echo "Error differs:"
    echo "actual: $actual"
    echo "expected: $expected"
    exit 1
fi

echo "Check multiple Hurl files"
actual=$(hurl tests_ok/hello.hurl tests_ok/hello.hurl)
expected="Hello World!Hello World!"
if [ "$actual" != "$expected" ]; then
    echo "Error differs:"
    echo "actual: $actual"
    echo "expected: $expected"
    exit 1
fi
