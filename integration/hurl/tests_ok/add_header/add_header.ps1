Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

$env:XDG_CONFIG_HOME=$PSScriptRoot/config
$env:HURL_HEADER='header-e: corge|header-f: grault'
hurl `
  --header 'header-b:baz' `
  --header 'header-c:qux' `
  tests_ok/add_header/add_header.hurl
