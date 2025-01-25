Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl --variable age=30 --variable strict=true --variable string_variable=`\ --variable key=dynamic_key tests_ok/post_json.hurl
