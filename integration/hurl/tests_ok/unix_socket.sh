#!/bin/bash
set -Eeuo pipefail
hurl --unix-socket build/unix_socket.sock --verbose tests_ok/unix_socket.hurl
