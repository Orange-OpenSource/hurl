Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

write-host -foregroundcolor Cyan "----- create windows64 zip package -----"

$actual_dir=(Get-Location).Path

# set vars
$hurl_package_version = Get-Content .\target\win-package\version.txt
$toolchain=((((rustup show active-toolchain) -Split " ")[0]) -Split "-",2)[1]
$zip_path = "hurl-${hurl_package_version}-${toolchain}.zip"
$temp_path = "$PSScriptRoot\..\..\target\tmp"

# create windows64 zip package
New-Item -ItemType Directory -Path $temp_path | Out-Null

cd $PSScriptRoot\..\..\target\win-package
Get-ChildItem -Path *.dll, *hurl.exe, *hurlfmt.exe, *.txt, ..\..\*.md | Copy-Item -Destination $temp_path
New-Item -ItemType Directory -Path $temp_path\completions | Out-Null
Copy-Item -Path ..\..\completions\*.ps1 -Recurse -Destination $temp_path\completions
Get-ChildItem -Path $temp_path

Compress-Archive -Path $temp_path\* -DestinationPath $zip_path
Remove-Item -Path $temp_path -Recurse -Force
Get-ChildItem .\*win64.zip

cd $actual_dir

