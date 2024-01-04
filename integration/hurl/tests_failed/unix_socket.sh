#!/bin/bash
set -Eeuo pipefail
hurl --unix-socket /unknown tests_ok/unix_socket.hurl
