Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --no-color tests_pty/stderr/stderr.hurl
