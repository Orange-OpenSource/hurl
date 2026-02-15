#!/bin/bash
set -Eeuo pipefail

hurl --no-pretty tests_ok/graphql/graphql.hurl
