#!/bin/bash
set -Eeuo pipefail

hurl --no-color --continue-on-error --secret name=a_secret_value tests_failed/secret/secret_errors.hurl
