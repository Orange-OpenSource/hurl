Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl --error-format long tests_failed/error_format_long.hurl
