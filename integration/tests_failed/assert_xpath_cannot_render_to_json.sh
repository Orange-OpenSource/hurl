#!/bin/bash
set -Eeuo pipefail
hurl tests_failed/assert_xpath_cannot_render_to_json.hurl
