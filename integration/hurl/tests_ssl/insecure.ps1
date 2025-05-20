Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --insecure --verbose tests_ssl/insecure.hurl
