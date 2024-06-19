#!/bin/bash
set -Eeuo pipefail

hurl --verbose tests_ok/delay_option.hurl
