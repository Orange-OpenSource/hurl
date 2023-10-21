Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
# GitHub runners only support IPV6 on macOS so we skip other OS (for the moment).
exit 255
hurl --very-verbose tests_ok/ip.hurl 2>&1 | grep 'Connected to google.com' | sed 's/^** //'
