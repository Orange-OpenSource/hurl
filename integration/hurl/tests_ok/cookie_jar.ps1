Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

if (Test-Path build/cookies.txt) {
    Remove-Item build/cookies.txt
}
hurl --cookie-jar build/cookies.txt --no-output tests_ok/cookie_jar.hurl
Write-Host (Get-Content build/cookies.txt -Raw) -NoNewLine
