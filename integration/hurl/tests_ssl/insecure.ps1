Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl tests_ssl/insecure.hurl --insecure --verbose
