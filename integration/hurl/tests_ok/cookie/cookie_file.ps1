Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --verbose --cookie tests_ok/cookie/cookie_file.cookies tests_ok/cookie/cookie_file.hurl
