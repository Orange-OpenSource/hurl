Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl tests_ok/header_option.hurl --header 'test: from-cli'
