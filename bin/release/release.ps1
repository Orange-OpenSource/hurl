Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

write-host -foregroundcolor Cyan "----- build release -----"

$actual_dir=(Get-Location).Path
$project_root_path=(Resolve-Path -LiteralPath $PSScriptRoot\..\..).path

# link libs
$actual_path=$env:Path
$vcpkg_dir=(Split-Path -Parent (Get-Command vcpkg).Source) -replace '\\','/'
$lib_dir="$vcpkg_dir/installed/x64-windows/bin"
$env:VCPKGRS_DYNAMIC=1
$env:BINDGEN_EXTRA_CLANG_ARGS="-I$vcpkg_dir/installed/x64-windows/include/libxml2 -v"
$env:Path="$lib_dir" + ";" + "$actual_path"

# build
cargo build --release --verbose --locked
if ($LASTEXITCODE) { Throw }

# create final package
$release_dir="$project_root_path\target\release"
$package_dir="$project_root_path\target\win-package"
New-Item -ItemType Directory -Force -Path $package_dir
Copy-Item -Path $lib_dir\libcurl.dll -Destination $package_dir
Copy-Item -Path $lib_dir\zlib1.dll -Destination $package_dir
Copy-Item -Path $lib_dir\nghttp2.dll -Destination $package_dir
Copy-Item -Path $lib_dir\libxml2.dll -Destination $package_dir
Copy-Item -Path $lib_dir\iconv-2.dll -Destination $package_dir
Copy-Item -Path $release_dir\hurl.exe -Destination $package_dir
Copy-Item -Path $release_dir\hurlfmt.exe -Destination $package_dir
((& $package_dir\hurl --version) -Split " ")[1] > $package_dir\version.txt
Get-Content $package_dir\version.txt

# add hurl to PATH
$registry_user_path=(Get-ItemProperty -Path 'HKCU:\Environment').Path
$registry_machine_path=(Get-ItemProperty -Path 'HKLM:\SYSTEM\CurrentControlSet\Control\Session Manager\Environment').Path
$env:Path = $actual_path + ";" + $package_dir + ";" + $registry_user_path + ";" + $registry_machine_path
sleep 10

# hurl infos
(Get-Command hurl).Path
(Get-Command hurlfmt).Path
hurl --version
if ($LASTEXITCODE) { Throw }
hurlfmt --version
if ($LASTEXITCODE) { Throw }

cd $actual_dir

