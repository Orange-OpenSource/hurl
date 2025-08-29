Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --include --pretty --color tests_ok/pretty/pretty.hurl
