#!/bin/bash
set -Eeuo pipefail

hurl --netrc-file tests_ok/netrc/netrc_file.netrc tests_ok/netrc/netrc.hurl
