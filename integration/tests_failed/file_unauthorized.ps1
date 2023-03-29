Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl tests_failed/file_unauthorized.hurl --fail-at-end
