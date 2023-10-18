Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
# GitHub runners only support IPV6 on macOS so we skip other OS (for the moment).
exit 255
hurl tests_ok/ip.hurl
