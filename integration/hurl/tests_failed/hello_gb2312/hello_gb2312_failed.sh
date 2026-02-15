#!/bin/bash
set -Eeuo pipefail

hurl --no-color tests_failed/hello_gb2312/hello_gb2312_failed.hurl
