#!/bin/bash
set -Eeuo pipefail

hurl --secret name=Alice tests_failed/assert_secret.hurl
