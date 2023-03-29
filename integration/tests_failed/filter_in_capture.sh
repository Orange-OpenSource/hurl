#!/bin/bash
set -Eeuo pipefail
hurl tests_failed/filter_in_capture.hurl
