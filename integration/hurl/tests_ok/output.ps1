Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'


if (Test-Path build/output.bin) {
    Remove-Item build/output.bin
}

hurl --output build/output.bin tests_ok/output.hurl
Write-Host (Get-Content build/output.bin -Raw) -NoNewLine
