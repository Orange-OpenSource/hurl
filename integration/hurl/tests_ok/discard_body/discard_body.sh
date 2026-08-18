#!/bin/bash
set -Eeuo pipefail

hurl --verbose --discard-body tests_ok/discard_body/discard_body.hurl
