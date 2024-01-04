Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

# The python test server for testing Unix Domain Sockets
# (../unix_socket/server.py) does not currently support AF_UNIX on Windows.
# See https://github.com/python/cpython/issues/77589
# Skip for now until this can be easily tested.
$ErrorActionPreference = 'Continue'
exit 255
