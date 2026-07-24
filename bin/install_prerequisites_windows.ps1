Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

write-host -foregroundcolor Cyan "----- install system prerequisites -----"

# install python 3.11
choco install --confirm python311
if ($LASTEXITCODE) { Throw }

# install proxy
echo "==== install Squid"
# Fix broken Chocolatey Squid package 4.14 because http://packages.diladele.com/squid/4.14/squid.msi moved to https://www.diladele.com/pkg/squid/4.14/squid.msi
Invoke-WebRequest https://community.chocolatey.org/api/v2/package/squid -OutFile squid.nupkg
Rename-Item squid.nupkg choco-squid.zip
Expand-Archive choco-squid.zip choco-squid
Select-String -Path .\choco-squid\tools\chocolateyinstall.ps1 -Pattern "diladele"
$file = '.\choco-squid\tools\chocolateyinstall.ps1'
$content = Get-Content $file -Raw
$content = $content.Replace(
    'http://packages.diladele.com/squid/4.14/squid.msi',
    'https://www.diladele.com/pkg/squid/4.14/squid.msi'
)
Set-Content -Path $file -Value $content -Encoding UTF8
cd .\choco-squid
choco pack
cd ..
choco install --debug --confirm squid --source=".\choco-squid" --install-arguments="'TARGETDIR=C:\'"
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

# update vcpkg install
$vcpkg_dir=((Get-command vcpkg).Source | Split-Path)
$lib_dir="$vcpkg_dir\installed\x64-windows\bin"
& "$vcpkg_dir\bootstrap-vcpkg.bat"
# Downgrade to 8.19.0 => https://github.com/Orange-OpenSource/hurl/issues/5105
git -C $vcpkg_dir checkout 4f326c4072038c8624c36a8ba5ed23f616adda53

# install libxml and libcurl
vcpkg install --recurse curl[core,sspi,http2,non-http,ssl]:x64-windows
vcpkg install --recurse libxml2[core,iconv]:x64-windows

vcpkg update
if ($LASTEXITCODE) { Throw }
vcpkg upgrade --no-dry-run
if ($LASTEXITCODE) { Throw }
vcpkg integrate install
if ($LASTEXITCODE) { Throw }
