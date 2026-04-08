Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --header 'Authorization: Bearer token123' tests_ok/unset_header/unset_header.hurl
