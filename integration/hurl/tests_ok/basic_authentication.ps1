Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl --user bob@email.com:secret --verbose tests_ok/basic_authentication.hurl
