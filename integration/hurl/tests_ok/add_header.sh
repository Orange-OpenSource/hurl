#!/bin/bash
set -Eeuo pipefail

hurl --header 'header-b:baz' --header 'header-c:qux' tests_ok/add_header.hurl
