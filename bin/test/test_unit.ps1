Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

write-host -foregroundcolor Cyan "----- unit tests  -----"

# exe dir
$original_env_path="$env:Path"

# lib dir
$vcpkg_dir=((Get-command vcpkg).Source | Split-Path)
$lib_dir="$vcpkg_dir\installed\x64-windows\bin"

# link libs
$env:Path = "$lib_dir" + ";" + "$original_env_path"

# execute test units
cargo test --release --tests
if ($LASTEXITCODE) { Throw }

# unlink libs
$env:Path = "$original_env_path"

