Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --cookie tests_ok/cookie/cookie_file.cookies --verbose tests_ok/cookie/cookie_file.hurl
