#!/bin/bash
set -Eeuo pipefail

hurl --header 'Authorization: Bearer token123' tests_ok/unset_header/unset_header.hurl
