powershell write-host -foregroundcolor Cyan "----- build release -----"

$actual_dir=(Get-Location).Path
$project_root_path=(Resolve-Path -LiteralPath $PSScriptRoot\..\..).path

# build
cargo build --release --verbose --locked

# create final package
$release_dir="$project_root_path\target\release"
$package_dir="$project_root_path\target\win-package"
New-Item -ItemType Directory -Force -Path $package_dir
Get-ChildItem -Path "$release_dir" -Recurse -Include *.dll -File | Copy-Item -Destination "$package_dir"
Get-ChildItem -Path "$release_dir" -Recurse -Include hurl*.exe -File | Copy-Item -Destination "$package_dir"
((& $package_dir\hurl --version) -Split " ")[1] > $package_dir\version.txt
Get-Content $package_dir\version.txt

# add hurl to PATH
$oldpath=(Get-ItemProperty -Path HKCU:\Environment -Name Path).Path
$newpath="$package_dir;$oldpath"
Set-ItemProperty -Path HKCU:\Environment -Name Path -Value $newpath
$env:Path = [System.Environment]::GetEnvironmentVariable("Path","User") + ";" + [System.Environment]::GetEnvironmentVariable("Path","Machine")
(Get-Command hurl).Path

# test hurl execution
hurl --version

cd $actual_dir
