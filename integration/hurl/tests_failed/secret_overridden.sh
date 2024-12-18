#!/bin/bash
set -Eeuo pipefail

hurl --secret name=a_secret_value tests_failed/secret_overridden.hurl
