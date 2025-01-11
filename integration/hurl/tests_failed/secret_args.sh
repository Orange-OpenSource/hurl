#!/bin/bash
set -Eeuo pipefail

hurl --secret foo=a --secret foo=b tests_failed/secret_args.hurl
