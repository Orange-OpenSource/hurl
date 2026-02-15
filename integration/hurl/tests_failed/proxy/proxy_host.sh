#!/bin/bash
set -Eeuo pipefail

hurl --no-color --proxy unknown tests_failed/proxy/proxy.hurl
