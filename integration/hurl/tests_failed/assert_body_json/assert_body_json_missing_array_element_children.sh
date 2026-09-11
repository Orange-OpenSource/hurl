#!/bin/bash
set -Eeuo pipefail

hurl tests_failed/assert_body_json/assert_body_json_missing_array_element_children.hurl
