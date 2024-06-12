Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

# We test that --output truncates an existing file then appends it.

echo "Not a response" > build/output.bin

hurl --output build/output.bin tests_ok/output.hurl tests_ok/output.hurl
Write-Host (Get-Content build/output.bin -Raw) -NoNewLine
