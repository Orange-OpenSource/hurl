#!/bin/bash
set -Eeuo pipefail

hurl --compressed tests_ok/compressed/compressed.hurl
