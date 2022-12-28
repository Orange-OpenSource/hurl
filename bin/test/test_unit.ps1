Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

powershell write-host -foregroundcolor Cyan "----- unit tests  -----"

# run test units
cargo test --release --features strict --tests
if ($LASTEXITCODE) { Throw }

# create test package
$project_root_path=(Resolve-Path -LiteralPath $PSScriptRoot\..\..).path
$release_dir="$project_root_path\target\release"
$package_dir="$project_root_path\target\test-package"
New-Item -ItemType Directory -Force -Path "$package_dir"
Get-ChildItem -Path $release_dir -Recurse -Include *.dll -File | Copy-Item -Destination $package_dir
Get-ChildItem -Path $release_dir -Recurse -Include hurl*.exe -File | Copy-Item -Destination $package_dir

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
