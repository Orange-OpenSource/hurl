Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

if (Test-Path build/stderr_piped.txt) {
    Remove-Item build/stderr_piped.txt
}
try {
    hurl tests_pty/stderr/stderr.hurl 2>build/stderr_piped.txt
} finally {
    $global:LASTEXITCODE = 0
    $ErrorActionPreference = 'Continue'
}

Write-Host (Get-Content build/stderr_piped.txt -Raw) -NoNewLine