#!/bin/bash
set -Eeuo pipefail

hurl tests_failed/invalid_protocol/invalid_protocol.hurl
