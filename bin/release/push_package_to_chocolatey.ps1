Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

write-host -foregroundcolor Cyan "----- Push package to chocolatey -----"

# Get dirs
$actual_dir=(Get-Location).Path
$project_root_path=(Resolve-Path -LiteralPath $PSScriptRoot\..\..).path

# Get args
if ($args.Count -lt 2) {
    Write-Host -ForegroundColor Red "Usage: push-package-to-chocolatey.ps1 <package version> <chocolatey token>, ex: push-package-to-chocolatey.ps1 4.2.0 dfdfdfdfdfdf2121d21d2f1df2"
    exit 1
}
$release = $args[0]
$token = $args[1]
if (-not (Test-Path variable:release)) {
    Write-Host -ForegroundColor Red "Usage: push-package-to-chocolatey.ps1 <package version> <chocolatey token>, ex: push-package-to-chocolatey.ps1 4.2.0 dfdfdfdfdfdf2121d21d2f1df2"
    exit 1
}
if (-not (Test-Path variable:token)) {
    Write-Host -ForegroundColor Red "Usage: push-package-to-chocolatey.ps1 <package version> <chocolatey token>, ex: push-package-to-chocolatey.ps1 4.2.0 dfdfdfdfdfdf2121d21d2f1df2"
    exit 1
}

write-host -foregroundcolor Cyan "# Get windows hurl package"
Invoke-WebRequest -UseBasicParsing "https://github.com/Orange-OpenSource/hurl/releases/download/${release}/hurl-${release}-x86_64-pc-windows-msvc.zip" -OutFile "C:\Windows\Temp\hurl-latest-win64.zip"
$hurl_sha=(Get-FileHash C:\Windows\Temp\hurl-latest-win64.zip).Hash

write-host -foregroundcolor Cyan "# Update choco package files"
cd ${project_root_path}\contrib\windows\windows_package_managers\chocolatey\hurl
(Get-Content -Path hurl.nuspec) | foreach{$_.replace('${hurl_latest_version}',${release})} | Set-Content hurl.nuspec
(Get-Content -Path tools\chocolateyinstall.ps1) | foreach{$_.replace('${hurl_latest_version}',${release})} | Set-Content tools\chocolateyinstall.ps1
(Get-Content -Path tools\chocolateyinstall.ps1) | foreach{$_.replace('${hurl_latest_sha}',${hurl_sha})} | Set-Content tools\chocolateyinstall.ps1
grep -R "${release}" *
grep -R "${hurl_sha}" *

write-host -foregroundcolor Cyan "# Test chocolatey installer"
choco pack
choco install hurl -s .
hurl --version
"GET https://hurl.dev","HTTP 200" | hurl --location --test

write-host -foregroundcolor Cyan "# Push package to official chocolatey repository"
choco apikey --key $token --source https://push.chocolatey.org/
choco push --source "https://push.chocolatey.org/"

cd $actual_dir

