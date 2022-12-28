Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

powershell write-host -foregroundcolor Cyan "----- install windows64 zip -----"

$actual_dir=(Get-Location).Path
$project_root_path=(Resolve-Path -LiteralPath $PSScriptRoot\..\..).path

# install windows64 zip
$zip_dir="$project_root_path\target\win-package"
$package_dir="$project_root_path\target\zip-package"
New-Item -ItemType Directory -Force -Path $package_dir
Expand-Archive -Path "$zip_dir\hurl-*.zip" -DestinationPath "$package_dir" -Force -Verbose

# add hurl to PATH
$registry_user_path=(Get-ItemProperty -Path 'HKCU:\Environment').Path
$registry_machine_path=(Get-ItemProperty -Path 'HKLM:\SYSTEM\CurrentControlSet\Control\Session Manager\Environment').Path
echo registry_user_path $registry_user_path
echo registry_machine_path $registry_machine_path
$env:Path = "$package_dir;$registry_user_path;$registry_machine_path"
sleep 10

# hurl infos
(Get-Command hurl).Path
(Get-Command hurlfmt).Path
hurl --version
if ($LASTEXITCODE) { Throw }
hurlfmt --version
if ($LASTEXITCODE) { Throw }

cd $actual_dir
