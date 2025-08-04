#!/bin/bash
set -Eeuo pipefail

hurl tests_failed/cookies/cookies.hurl
