#!/bin/bash
set -Eeuo pipefail

hurl tests_ok/retry/retry_until_200.hurl
