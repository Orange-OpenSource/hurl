Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl tests_ok/connect_to.hurl --connect-to foo.com:80:localhost:8000 --connect-to bar.com:80:localhost:8000 --connect-to baz.com:80:localhost:8000 --verbose
