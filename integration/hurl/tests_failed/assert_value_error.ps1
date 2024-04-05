Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

$out=hurl --json tests_failed/assert_value_error.hurl
$exit_code="$lastexitcode"
echo "$out" | jq
exit "$exit_code"