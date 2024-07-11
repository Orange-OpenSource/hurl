#!/bin/bash
set -Eeuo pipefail

hurl tests_ok/max_redirect_infinite_option.hurl
