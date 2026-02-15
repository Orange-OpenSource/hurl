Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

if (Test-Path build/no_cookie_store_output.txt) {
    Remove-Item build/no_cookie_store_output.txt
}

hurl --no-color --verbose --no-cookie-store --cookie-jar build/no_cookie_store_output.txt tests_ok/no_cookie_store/no_cookie_store.hurl
Write-Host (Get-Content build/no_cookie_store_output.txt -Raw) -NoNewLine