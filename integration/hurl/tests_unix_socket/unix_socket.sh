#!/bin/bash
set -Eeuo pipefail
hurl --unix-socket build/unix_socket.sock --verbose tests_unix_socket/unix_socket.hurl
