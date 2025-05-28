Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
if (Test-Path build/output_request_1.bin) {
    Remove-Item build/output_request_1.bin
}
if (Test-Path build/output_request_2.bin) {
    Remove-Item build/output_request_2.bin
}
hurl --no-output --file-root build tests_ok/output_option.hurl
Write-Host (Get-Content build/output_request_1.bin -Raw) -NoNewLine
Write-Host (Get-Content build/output_request_2.bin -Raw) -NoNewLine
