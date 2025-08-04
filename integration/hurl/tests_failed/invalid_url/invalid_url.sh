#!/bin/bash
set -Eeuo pipefail

hurl tests_failed/invalid_url/invalid_url.hurl
