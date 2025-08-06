#!/bin/bash
set -Eeuo pipefail

hurl --proxy unknown tests_failed/proxy/proxy.hurl
