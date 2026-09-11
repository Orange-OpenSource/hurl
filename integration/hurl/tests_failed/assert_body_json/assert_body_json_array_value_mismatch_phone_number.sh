#!/bin/bash
set -Eeuo pipefail

hurl tests_failed/assert_body_json/assert_body_json_array_value_mismatch_phone_number.hurl
