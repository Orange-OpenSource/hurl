Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

write-host -foregroundcolor Cyan "----- install system prerequisites -----"

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
Set-ItemProperty -Path HKCU:\Environment -Name VCPKGRS_DYNAMIC -Value "1"
$env:VCPKGRS_DYNAMIC = [System.Environment]::GetEnvironmentVariable("VCPKGRS_DYNAMIC","User")
if ($LASTEXITCODE) { Throw }

# update pip
python -m pip install --upgrade pip --break-system-packages --quiet
if ($LASTEXITCODE) { Throw }

# install proxy
choco install --confirm squid --install-arguments="'TARGETDIR=C:\'"
if ($LASTEXITCODE) { Throw }
Get-ChildItem -Force C:\Squid\bin
C:\Squid\bin\squid --version
if ($LASTEXITCODE) { Throw }
