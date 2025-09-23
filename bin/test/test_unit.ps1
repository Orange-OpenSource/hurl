Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

write-host -foregroundcolor Cyan "----- unit tests  -----"

# link libs
$actual_path=$env:Path
$vcpkg_dir=(Split-Path -Parent (Get-Command vcpkg).Source) -replace '\\','/'
$lib_dir="$vcpkg_dir/installed/x64-windows/bin"
$env:VCPKGRS_DYNAMIC=1
$env:BINDGEN_EXTRA_CLANG_ARGS="-I$vcpkg_dir/installed/x64-windows/include/libxml2 -v"
$env:Path="$lib_dir" + ";" + "$actual_path"

# execute test units
cargo test --release --tests
if ($LASTEXITCODE) { Throw }

