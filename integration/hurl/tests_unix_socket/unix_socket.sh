#!/bin/bash
set -Eeuo pipefail

hurl --no-color \
  --unix-socket build/unix_socket.sock \
  --verbose \
  tests_unix_socket/unix_socket.hurl
