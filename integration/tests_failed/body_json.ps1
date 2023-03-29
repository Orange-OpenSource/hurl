Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl tests_failed/body_json.hurl --variable success=invalid
