#!/bin/bash
set -Eeuo pipefail

hurl --ipv4 tests_failed/ipv4/ipv4.hurl
