#!/bin/bash
set -Eeuo pipefail

hurl --no-output tests_ok/rawbytes/rawbytes.hurl
