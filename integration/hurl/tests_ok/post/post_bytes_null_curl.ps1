Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --curl build/post_bytes_null.curl --no-output tests_ok/post/post_bytes_null.hurl

Write-Host (Get-Content build/post_bytes_null.curl -Raw) -NoNewLine