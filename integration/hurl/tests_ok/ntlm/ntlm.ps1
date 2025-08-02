Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --ntlm -u ":" tests_ok/ntlm/ntlm.hurl
