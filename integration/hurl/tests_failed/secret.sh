#!/bin/bash
set -Eeuo pipefail

hurl --secret name=Alice tests_failed/secret.hurl
