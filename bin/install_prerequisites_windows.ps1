Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

powershell write-host -foregroundcolor Cyan "----- install system prerequisites -----"

# install libxml and libcurl[openssl]
$vcpkg_dir=(Get-command vcpkg).Source
if (Test-Path $vcpkg_dir\installed\x64-windows\bin\libssl-3-x64.dll) {echo "curl[openssl] already installed"} else {vcpkg install --recurse curl[openssl]:x64-windows}
if ($LASTEXITCODE) { Throw }
if (Test-Path $vcpkg_dir\installed\x64-windows\bin\libxml2.dll) {echo "libxml2 already installed"} else {vcpkg install --recurse libxml2:x64-windows}
if ($LASTEXITCODE) { Throw }
vcpkg update
if ($LASTEXITCODE) { Throw }
vcpkg upgrade --no-dry-run
if ($LASTEXITCODE) { Throw }
vcpkg integrate install
if ($LASTEXITCODE) { Throw }

# update pip
python -m pip install --upgrade pip --quiet
if ($LASTEXITCODE) { Throw }
