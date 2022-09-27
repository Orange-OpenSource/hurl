powershell write-host -foregroundcolor Cyan "----- unit tests  -----"

# run test units
cargo test --release --features strict --tests

# create test package
$project_root_path=(Resolve-Path -LiteralPath $PSScriptRoot\..\..).path
$release_dir="$project_root_path\target\release"
$package_dir="$project_root_path\target\test-package"
New-Item -ItemType Directory -Force -Path "$package_dir"
Get-ChildItem -Path $release_dir -Recurse -Include *.dll -File | Copy-Item -Destination $package_dir
Get-ChildItem -Path $release_dir -Recurse -Include hurl*.exe -File | Copy-Item -Destination $package_dir

# add hurl to PATH
$oldpath=(Get-ItemProperty -Path HKCU:\Environment -Name Path).Path
$newpath="$package_dir;$oldpath"
Set-ItemProperty -Path HKCU:\Environment -Name Path -Value $newpath
$env:Path =  [System.Environment]::GetEnvironmentVariable("Path","User") + ";" + [System.Environment]::GetEnvironmentVariable("Path","Machine")
