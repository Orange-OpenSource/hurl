#!/bin/bash
set -Eeuo pipefail
hurl tests_failed/file_read_access.hurl
