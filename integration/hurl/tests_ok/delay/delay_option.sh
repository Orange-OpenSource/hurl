#!/bin/bash
set -Eeuo pipefail

hurl --verbose tests_ok/delay/delay_option.hurl
