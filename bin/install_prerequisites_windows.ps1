Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

write-host -foregroundcolor Cyan "----- install system prerequisites -----"

# update vcpkg install
$vcpkg_dir=((Get-command vcpkg).Source | Split-Path)
$lib_dir="$vcpkg_dir\installed\x64-windows\bin"
git -C $vcpkg_dir pull

# install libxml and libcurl[openssl]
vcpkg install --recurse curl[core,non-http,schannel,ssl,sspi,http2]:x64-windows
vcpkg install --recurse libxml2[core,iconv]:x64-windows
vcpkg update
if ($LASTEXITCODE) { Throw }
vcpkg upgrade --no-dry-run
if ($LASTEXITCODE) { Throw }
vcpkg integrate install
if ($LASTEXITCODE) { Throw }
Set-ItemProperty -Path HKCU:\Environment -Name VCPKGRS_DYNAMIC -Value "1"
$env:VCPKGRS_DYNAMIC = [System.Environment]::GetEnvironmentVariable("VCPKGRS_DYNAMIC","User")
if ($LASTEXITCODE) { Throw }

# install python 3.11
choco install --confirm python311
if ($LASTEXITCODE) { Throw }

# install proxy
echo "==== install Squid"
choco install --confirm squid --install-arguments="'TARGETDIR=C:\'"
if ($LASTEXITCODE) { Throw }
echo "==== create log dir integration\build"
New-Item -ItemType Directory -Path integration\build -Force
echo "==== Squid service status"
sc queryex squidsrv | tee -Append -filepath integration\build\proxy.log
echo "==== Squid process status"
Get-Process | Where {$_.Name -eq "Squid"} | tee -Append -filepath integration\build\proxy.log
echo "==== Squid version"
C:\Squid\bin\squid --version | tee -Append -filepath integration\build\proxy.log
echo "==== stop Squid service and kill child process"
taskkill /f /fi "SERVICES eq squidsrv" 2>&1 | tee -Append -filepath integration\build\proxy.log
if ($LASTEXITCODE) { Throw }
taskkill /f /IM squid.exe 2>&1 | tee -Append -filepath integration\build\proxy.log
if ($LASTEXITCODE) { Throw }
echo "==== Squid service status"
sc queryex squidsrv | tee -Append -filepath integration\build\proxy.log
echo "==== Squid process status"
Get-Process | Where {$_.Name -eq "Squid"} | tee -Append -filepath integration\build\proxy.log

# install jq
echo "==== install jq"
choco install --confirm jq
if ($LASTEXITCODE) { Throw }
