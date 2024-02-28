Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
if (Test-Path build/fileroot.bin) {
    Remove-Item build/fileroot.bin
}
hurl --file-root build/ tests_ok/fileroot.hurl
