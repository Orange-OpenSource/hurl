Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --variable success=invalid tests_failed/body/body_json.hurl
