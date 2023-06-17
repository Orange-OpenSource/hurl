Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
if (Test-Path tests_ok/output.bin) {
    Remove-Item tests_ok/output.bin
}
hurl --output tests_ok/output.bin tests_ok/output.hurl
Write-Host (Get-Content tests_ok/output.bin -Raw) -NoNewLine
