#!/bin/bash
set -Eeuo pipefail

hurl tests_ok/post_multilines.hurl --verbose
