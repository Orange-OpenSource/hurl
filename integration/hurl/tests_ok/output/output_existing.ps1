Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

# We test that --output truncates an existing file then appends it.

echo "Not a response" > build/output_existing.bin

hurl --output build/output_existing.bin tests_ok/output_existing.hurl tests_ok/output_existing.hurl
Write-Host (Get-Content build/output_existing.bin -Raw) -NoNewLine
