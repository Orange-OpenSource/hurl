#!/bin/bash
set -Eeuo pipefail

hurl --output /dev/null tests_ok/get_large/get_large.hurl
