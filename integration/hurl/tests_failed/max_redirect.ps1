Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl tests_failed/max_redirect.hurl --location --max-redirs 5
