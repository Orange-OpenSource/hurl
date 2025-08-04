#!/bin/bash
set -Eeuo pipefail

hurl tests_failed/hello_gb2312/hello_gb2312_failed.hurl
