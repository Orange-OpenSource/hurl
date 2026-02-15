Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --no-color tests_ok/deprecated/deprecated.hurl
