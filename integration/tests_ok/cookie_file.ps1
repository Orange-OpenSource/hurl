Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl tests_ok/cookie_file.hurl --cookie tests_ok/cookie_file.cookies --verbose
