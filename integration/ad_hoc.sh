#!/bin/bash
# Add ad-hoc tests that can't be easily added in tests_ok/ nor tests_failed/

function assert_equals() {
  if [ "$1" != "$2" ]; then
      echo "Error differs:"
      echo "actual: $1"
      echo "expected: $2"
      exit 1
  fi
}

echo "Check file not found error"
actual=$(hurl does_not_exist.hurl 2>&1)
expected="error: hurl: cannot access 'does_not_exist.hurl': No such file or directory"
assert_equals "$actual" "$expected"

echo "Check multiple Hurl files"
actual=$(hurl tests_ok/hello.hurl tests_ok/hello.hurl)
expected="Hello World!Hello World!"
assert_equals "$actual" "$expected"

echo "Check stdin"
actual=$(echo 'GET http://localhost:8000/hello' | hurl)
expected="Hello World!"
assert_equals "$actual" "$expected"
