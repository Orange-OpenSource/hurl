Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

write-host -foregroundcolor Cyan "----- install system prerequisites -----"

# Update vcpkg install
$vcpkg_dir=((Get-command vcpkg).Source | Split-Path)
$lib_dir="$vcpkg_dir\installed\x64-windows\bin"

# Ensure cached vcpkg repo is clean before updating
git -C $vcpkg_dir reset --hard
if ($LASTEXITCODE) { Throw }
git -C $vcpkg_dir clean -fdx
if ($LASTEXITCODE) { Throw }

# Deterministic update (avoids merge conflicts from dirty state)
git -C $vcpkg_dir fetch --tags origin
if ($LASTEXITCODE) { Throw }
git -C $vcpkg_dir reset --hard origin/master
if ($LASTEXITCODE) { Throw }

# Re-bootstrap after cleanup because git clean -fdx removes vcpkg.exe
& "$vcpkg_dir\bootstrap-vcpkg.bat"
if ($LASTEXITCODE) { Throw }

# Install libxml and libcurl
vcpkg install --recurse "curl[core,sspi,http2,non-http,ssl]:x64-windows"
vcpkg install --recurse "libxml2[core,iconv]:x64-windows"

vcpkg update
if ($LASTEXITCODE) { Throw }
vcpkg upgrade --no-dry-run
if ($LASTEXITCODE) { Throw }
vcpkg integrate install
if ($LASTEXITCODE) { Throw }

# Install python 3.11
choco install --confirm python311
if ($LASTEXITCODE) { Throw }

# Install proxy
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

# Install jq
echo "==== install jq"
choco install --confirm jq
if ($LASTEXITCODE) { Throw }
