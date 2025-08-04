#!/bin/bash
set -Eeuo pipefail

hurl tests_failed/output/output_unauthorized.hurl
