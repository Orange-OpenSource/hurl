Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

$out=hurl --no-color --json tests_failed/assert_value_error/assert_value_error.hurl
$exit_code="$lastexitcode"
echo "$out" | jq --monochrome-output
exit "$exit_code"