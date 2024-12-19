#!/bin/bash
set -Eeuo pipefail

hurl tests_ok/header_option.hurl --header 'test: from-cli'
