Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --no-color --variable success=invalid tests_failed/body/body_json.hurl
