#!/bin/bash
set -Eeuo pipefail

hurl --continue-on-error --secret name=a_secret_value tests_failed/secret_errors.hurl
