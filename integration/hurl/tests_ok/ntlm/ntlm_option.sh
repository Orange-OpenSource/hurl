#!/bin/bash
set -Eeuo pipefail

hurl --verbose tests_ok/ntlm/ntlm_option.hurl
