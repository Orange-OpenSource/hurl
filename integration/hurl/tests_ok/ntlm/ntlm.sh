#!/bin/bash
set -Eeuo pipefail

hurl --ntlm -u ":" tests_ok/ntlm/ntlm.hurl
