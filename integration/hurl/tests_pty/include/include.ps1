Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --include tests_pty/include/include.hurl
