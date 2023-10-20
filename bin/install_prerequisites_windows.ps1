Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

write-host -foregroundcolor Cyan "----- install system prerequisites -----"

# update vcpkg install
$vcpkg_dir=((Get-command vcpkg).Source | Split-Path)
$lib_dir="$vcpkg_dir\installed\x64-windows\bin"
git -C $vcpkg_dir pull

# install libxml and libcurl[openssl]
vcpkg install --recurse --x-use-aria2 curl[core,non-http,schannel,ssl,sspi,http2]:x64-windows  || true
vcpkg install --recurse --x-use-aria2 libxml2[core,iconv]:x64-windows || true
vcpkg update
if ($LASTEXITCODE) { Throw }
vcpkg upgrade --no-dry-run
if ($LASTEXITCODE) { Throw }
vcpkg integrate install
if ($LASTEXITCODE) { Throw }
Set-ItemProperty -Path HKCU:\Environment -Name VCPKGRS_DYNAMIC -Value "1"
$env:VCPKGRS_DYNAMIC = [System.Environment]::GetEnvironmentVariable("VCPKGRS_DYNAMIC","User")
if ($LASTEXITCODE) { Throw }

# update pip
python -m pip install --upgrade pip --quiet
if ($LASTEXITCODE) { Throw }

# install proxy
choco install --confirm squid --install-arguments="'TARGETDIR=C:\'"
if ($LASTEXITCODE) { Throw }
Get-ChildItem -Force C:\Squid\bin
C:\Squid\bin\squid --version
if ($LASTEXITCODE) { Throw }
