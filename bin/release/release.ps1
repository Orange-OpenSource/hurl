Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

powershell write-host -foregroundcolor Cyan "----- build release -----"

$actual_dir=(Get-Location).Path
$project_root_path=(Resolve-Path -LiteralPath $PSScriptRoot\..\..).path

# build
cargo build --release --verbose --locked
if ($LASTEXITCODE) { Throw }

# create final package
$release_dir="$project_root_path\target\release"
$package_dir="$project_root_path\target\win-package"
New-Item -ItemType Directory -Force -Path $package_dir
Get-ChildItem -Path $release_dir -Recurse -Include *.dll -File | Copy-Item -Destination "$package_dir"
Get-ChildItem -Path $release_dir -Recurse -Include hurl*.exe -File | Copy-Item -Destination "$package_dir"
((& $package_dir\hurl --version) -Split " ")[1] > $package_dir\version.txt
Get-Content $package_dir\version.txt

# add hurl to PATH
$registry_user_path=(Get-ItemProperty -Path 'HKCU:\Environment').Path
$registry_machine_path=(Get-ItemProperty -Path 'HKLM:\SYSTEM\CurrentControlSet\Control\Session Manager\Environment').Path
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
