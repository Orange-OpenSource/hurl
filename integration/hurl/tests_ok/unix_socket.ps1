Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

# The python test server for testing Unix Domain Sockets
# (../tests_unix_socket/server.py) does not currently support AF_UNIX on Windows.
# See https://github.com/python/cpython/issues/77589
# Skip for now until this can be easily tested.
#hurl --unix-socket build/unix_socket.sock --verbose tests_ok/unix_socket.hurl
$ErrorActionPreference = 'Continue'
exit 255
