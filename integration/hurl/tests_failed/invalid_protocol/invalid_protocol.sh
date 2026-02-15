#!/bin/bash
set -Eeuo pipefail

hurl --no-color tests_failed/invalid_protocol/invalid_protocol.hurl
