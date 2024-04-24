Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

write-host -foregroundcolor Cyan "----- Push package to winget -----"

# Get args
if ($args.Count -lt 2) {
    Write-Host -ForegroundColor Red "Usage: push-package-to-winget.ps1 <package version> <winget token>, ex: push-package-to-winget.ps1 4.2.0 dfdfdfdfdfdf2121d21d2f1df2"
    exit 1
}
$release = $args[0]
$token = $args[1]
if (-not (Test-Path variable:release)) {
    Write-Host -ForegroundColor Red "Usage: push-package-to-winget.ps1 <package version> <winget token>, ex: push-package-to-winget.ps1 4.2.0 dfdfdfdfdfdf2121d21d2f1df2"
    exit 1
}
if (-not (Test-Path variable:token)) {
    Write-Host -ForegroundColor Red "Usage: push-package-to-winget.ps1 <package version> <winget token>, ex: push-package-to-winget.ps1 4.2.0 dfdfdfdfdfdf2121d21d2f1df2"
    exit 1
}

.\wingetcreate.exe update --submit --token "$token" --urls "https://github.com/Orange-OpenSource/hurl/releases/download/$release/hurl-$release-x86_64-pc-windows-msvc-installer.exe|x64" --version "$release" "Orange-OpenSource.Hurl"
