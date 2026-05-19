Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --header foo:FOO tests_ok/no_header/no_header_option.hurl
