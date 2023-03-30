Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl tests_ok/basic_authentication.hurl --user bob@email.com:secret --verbose
