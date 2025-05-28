#!/bin/bash
set -Eeuo pipefail

hurl --verbose tests_ok/reason/reason.hurl
