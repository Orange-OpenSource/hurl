Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

write-host -foregroundcolor Cyan "----- create windows64 zip package -----"

$actual_dir=(Get-Location).Path

# get built hurl version
$hurl_package_version = Get-Content .\target\win-package\version.txt
$toolchain=((((rustup show active-toolchain) -Split " ")[0]) -Split "-",2)[1]

# create windows64 zip package
cd $PSScriptRoot\..\..\target\win-package
Get-ChildItem -Path *.dll, *hurl.exe, *hurlfmt.exe, *.txt, ../../*.md, ../../completions/_hurl.ps1 | Compress-Archive -DestinationPath hurl-${hurl_package_version}-${toolchain}.zip
Get-ChildItem .\*win64.zip

cd $actual_dir

