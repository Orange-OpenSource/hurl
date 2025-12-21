Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --digest -u "username:password" tests_ok/digest/digest.hurl
