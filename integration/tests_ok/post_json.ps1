Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl tests_ok/post_json.hurl --variable age=30 --variable strict=true --variable string_variable='\' --verbose
