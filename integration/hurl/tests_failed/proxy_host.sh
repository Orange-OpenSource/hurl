#!/bin/bash
set -Eeuo pipefail

hurl --proxy unknown tests_ok/hello/hello.hurl
