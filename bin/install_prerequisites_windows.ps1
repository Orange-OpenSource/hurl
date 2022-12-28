Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

powershell write-host -foregroundcolor Cyan "----- install system prerequisites -----"

# install libxml and libcurl[openssl]
$vcpkg_dir=(Get-command vcpkg).Source

if (Test-Path $vcpkg_dir\installed\x64-windows\lib\libcurl.lib) {echo "curl already installed"} else {vcpkg install curl:x64-windows}
if ($LASTEXITCODE) { Throw }
if (Test-Path $vcpkg_dir\installed\x64-windows\lib\libxml2.lib) {echo "libxml2 already installed"} else {vcpkg install libxml2:x64-windows}
if ($LASTEXITCODE) { Throw }

vcpkg update
if ($LASTEXITCODE) { Throw }
vcpkg upgrade
if ($LASTEXITCODE) { Throw }
vcpkg integrate install
if ($LASTEXITCODE) { Throw }

# update pip
python -m pip install --upgrade pip --quiet
if ($LASTEXITCODE) { Throw }

