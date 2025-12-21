#!/bin/bash
set -Eeuo pipefail

hurl --verbose tests_ok/digest/digest_option.hurl
