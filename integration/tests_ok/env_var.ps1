Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
$env:HURL_name = 'Bob'
hurl tests_ok/env_var.hurl
