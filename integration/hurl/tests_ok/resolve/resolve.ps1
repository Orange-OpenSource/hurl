Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --resolve foo.com:8000:127.0.0.1 --resolve bar.com:8000:127.0.0.1 --resolve baz.com:8000:127.0.0.1 tests_ok/resolve/resolve.hurl
