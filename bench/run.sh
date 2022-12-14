#!/bin/bash
set -Eeuo pipefail

time hurl tests/hello_1000.hurl
