Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

write-host -foregroundcolor Cyan "----- install tests prerequisites -----"

$actual_dir=(Get-Location).Path
$project_root_path=(Resolve-Path -LiteralPath $PSScriptRoot\..\..).path

# install python libs
python -m pip install --requirement $project_root_path\bin\requirements-frozen.txt
if ($LASTEXITCODE) { Throw }

# start mock servers
cd $project_root_path\integration\hurl
New-Item -ItemType Directory -Force -Path build

python server.py 2>&1 > build\server.log &
if ($LASTEXITCODE) { Throw }
sleep 5
if (netstat -ano | Select-String LISTENING | Select-string 127.0.0.1:8000) {write-host -foregroundcolor Green "server is up"} else {write-host -foregroundcolor Red "server is down" ; cat build\server.log ; exit 1}

python tests_ssl/ssl_server.py 8001 tests_ssl/certs/server/cert.selfsigned.pem false 2>&1 > build\server-ssl-selfsigned.log &
if ($LASTEXITCODE) { Throw }
sleep 5
if (netstat -ano | Select-String LISTENING | Select-string 127.0.0.1:8001) {write-host -foregroundcolor Green "server-ssl-selfsigned up"} else {write-host -foregroundcolor Red "server-ssl-selfsigned is down" ; cat build\server-ssl-selfsigned.log ; exit 1}

python tests_ssl/ssl_server.py 8002 tests_ssl/certs/server/cert.pem false 2>&1 > build\server-ssl-signedbyca.log &
if ($LASTEXITCODE) { Throw }
sleep 5
if (netstat -ano | Select-String LISTENING | Select-string 127.0.0.1:8002) {write-host -foregroundcolor Green "server-ssl-signedbyca up"} else {write-host -foregroundcolor Red "server-ssl-signedbyca is down" ; cat build\server-ssl-signedbyca.log ; exit 1}

python tests_ssl/ssl_server.py 8003 tests_ssl/certs/server/cert.pem true 2>&1 > build\server-ssl-client-authent.log &
if ($LASTEXITCODE) { Throw }
sleep 5
if (netstat -ano | Select-String LISTENING | Select-string 127.0.0.1:8003) {write-host -foregroundcolor Green "server-ssl-client-authent up"} else {write-host -foregroundcolor Red "server-ssl-client-authent is down" ; cat build\server-ssl-client-authent.log ; exit 1}

Get-ChildItem -Force C:\Squid\bin
write-output "cache deny all" "cache_log /dev/null" "access_log /dev/null" "http_access allow all" "http_port 0.0.0.0:3128" "request_header_add From-Proxy Hello" "reply_header_add From-Proxy Hello" > squid.conf
C:\Squid\bin\squid -d 2 -N -f squid.conf 2>&1 | tee -Append -filepath build\proxy.log &
if ($LASTEXITCODE) { Throw }
sleep 5
if (netstat -ano | Select-String LISTENING | Select-string 0.0.0.0:3128) {write-host -foregroundcolor Green "proxy is up"} else {write-host -foregroundcolor Red "proxy is down" ; cat build\proxy.log ; exit 1}

cd $actual_dir

