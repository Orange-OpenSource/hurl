#!/bin/bash
set -Eeuo pipefail

hurl tests_failed/assert_xpath/assert_xpath.hurl
