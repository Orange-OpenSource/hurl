Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

write-host -foregroundcolor Cyan "----- install system prerequisites -----"

# install python 3.11
choco install --confirm --no-progress python311
if ($LASTEXITCODE) { Throw }

# install proxy
echo "==== install mitmproxy"
$mitmproxy_version = '12.3.1'
$mitmproxy_zip = Join-Path $env:TEMP 'mitmproxy.zip'
Invoke-WebRequest "https://downloads.mitmproxy.org/$mitmproxy_version/mitmproxy-$mitmproxy_version-windows-x86_64.zip" -OutFile $mitmproxy_zip
Expand-Archive -Path $mitmproxy_zip -DestinationPath C:\mitmproxy -Force
echo "==== create log dir integration\build"
New-Item -ItemType Directory -Path integration\build -Force
echo "==== mitmproxy version"
C:\mitmproxy\mitmdump.exe --version | tee -Append -filepath integration\build\proxy.log
if ($LASTEXITCODE) { Throw "mitmproxy installation failed" }

# install jq
echo "==== install jq"
choco install --confirm --no-progress jq
if ($LASTEXITCODE) { Throw }

# update vcpkg install
$vcpkg_dir=((Get-command vcpkg).Source | Split-Path)
$lib_dir="$vcpkg_dir\installed\x64-windows\bin"
git -C $vcpkg_dir pull
& "$vcpkg_dir\bootstrap-vcpkg.bat"
vcpkg upgrade --no-dry-run
if ($LASTEXITCODE) { Throw }

# install libxml and libcurl
vcpkg install --recurse curl[core,sspi,http2,ssl]:x64-windows
if ($LASTEXITCODE) { Throw "curl installation failed" }
vcpkg install --recurse libxml2[core,iconv]:x64-windows
if ($LASTEXITCODE) { Throw "libxml2 installation failed" }

vcpkg integrate install
if ($LASTEXITCODE) { Throw }
