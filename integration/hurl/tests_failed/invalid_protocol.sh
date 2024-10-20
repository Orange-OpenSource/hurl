#!/bin/bash
set -Eeuo pipefail
hurl tests_failed/invalid_protocol.hurl
