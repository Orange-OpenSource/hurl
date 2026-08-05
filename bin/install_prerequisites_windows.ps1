Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

write-host -foregroundcolor Cyan "----- install system prerequisites -----"

# install python 3.11
choco install --confirm python311
if ($LASTEXITCODE) { Throw }

# install proxy
echo "==== install Squid"
$squid_msi = Join-Path $env:TEMP 'squid.msi'
Invoke-WebRequest 'https://www.diladele.com/pkg/squid/4.14/squid.msi' -OutFile $squid_msi
$process = Start-Process -FilePath 'msiexec.exe' -ArgumentList @(
    '/i', $squid_msi,
    '/qn',
    '/norestart',
    'TARGETDIR=C:\'
) -Wait -PassThru
if ($process.ExitCode) { Throw "Squid installation fails with exit code $($process.ExitCode)" }
echo "==== create log dir integration\build"
New-Item -ItemType Directory -Path integration\build -Force
echo "==== Squid service status"
sc queryex squidsrv | tee -Append -filepath integration\build\proxy.log
echo "==== Squid process status"
Get-Process | Where {$_.Name -eq "Squid"} | tee -Append -filepath integration\build\proxy.log
echo "==== Squid version"
C:\Squid\bin\squid --version | tee -Append -filepath integration\build\proxy.log
echo "==== stop Squid service and kill child process"
Get-Service -Name 'squidsrv' -ErrorAction SilentlyContinue | ForEach-Object {
    if ($_.Status -ne 'Stopped') {
        Stop-Service -Name $_.Name -Force -ErrorAction SilentlyContinue
    }
}
Get-Process -Name 'squid' -ErrorAction SilentlyContinue | Stop-Process -Force

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
