Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --cookie tests_ok/cookie/cookie_file.cookies tests_ok/cookie/cookie_file.hurl
hurl --cookie 'cookie1=valueA' tests_ok/cookie/cookie_file.hurl
