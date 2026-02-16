Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --no-color tests_pty/binary_output/binary_output.hurl
