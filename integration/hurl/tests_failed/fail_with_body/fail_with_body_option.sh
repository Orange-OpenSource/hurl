#!/bin/bash
set -Eeuo pipefail

hurl tests_failed/fail_with_body/fail_with_body_option.hurl
