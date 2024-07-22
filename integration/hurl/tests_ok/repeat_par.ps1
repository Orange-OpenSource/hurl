Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
if (Test-Path build/repeat/tap.txt) {
    Remove-Item build/repeat/tap.txt
}

hurl --repeat 4 --parallel --report-tap build/repeat/tap.txt --no-output `
  tests_ok/repeat_a.hurl `
  tests_ok/repeat_b.hurl `
  tests_ok/repeat_c.hurl

Write-Host (Get-Content build/repeat/tap.txt -Raw) -NoNewLine