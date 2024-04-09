Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl --user bob@email.com:secret tests_ok/basic_authentication.hurl
