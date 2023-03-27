Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

powershell write-host -foregroundcolor Cyan "----- install system prerequisites -----"

# update vcpkg install
git -C ((Get-command vcpkg).Source | Split-Path) pull

# install libxml and libcurl[openssl]
$vcpkg_dir=(Get-command vcpkg).Source

vcpkg install curl:x64-windows || true
vcpkg install libxml2:x64-windows || true

vcpkg update
if ($LASTEXITCODE) { Throw }
vcpkg upgrade --no-dry-run
if ($LASTEXITCODE) { Throw }
vcpkg integrate install
if ($LASTEXITCODE) { Throw }

# update pip
python -m pip install --upgrade pip --quiet
if ($LASTEXITCODE) { Throw }
