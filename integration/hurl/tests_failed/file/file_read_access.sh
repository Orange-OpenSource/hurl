#!/bin/bash
set -Eeuo pipefail

hurl --no-color tests_failed/file/file_read_access.hurl
