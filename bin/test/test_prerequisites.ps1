Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

powershell write-host -foregroundcolor Cyan "----- install tests prerequisites -----"

$actual_dir=(Get-Location).Path
$project_root_path=(Resolve-Path -LiteralPath $PSScriptRoot\..\..).path

# install python libs
pip3 install --requirement $project_root_path\bin\requirements-frozen.txt
if ($LASTEXITCODE) { Throw }

# start mock servers
cd $project_root_path\integration
New-Item -ItemType Directory -Force -Path build

#Start-Process powershell -WindowStyle Hidden { mitmdump --listen-port 8888 --modify-header "/From-Proxy/Hello" 2>&1 > build\mitmdump.log }
#if ($LASTEXITCODE) { Throw }
#sleep 5
#if (netstat -ano | Select-String LISTENING | Select-string 0.0.0.0:8888) {powershell write-host -foregroundcolor Green "mitmdump is up"} else {powershell write-host -foregroundcolor Red "mitmdump is down" ; exit 1}

Start-Process powershell -WindowStyle Hidden { python server.py 2>&1 > build\server.log }
if ($LASTEXITCODE) { Throw }
sleep 5
if (netstat -ano | Select-String LISTENING | Select-string 127.0.0.1:8000) {powershell write-host -foregroundcolor Green "server is up"} else {powershell write-host -foregroundcolor Red "server is down" ; exit 1}

Start-Process powershell -WindowStyle Hidden { python ssl/server.py 8001 ssl/server/cert.selfsigned.pem false 2>&1 > build\server-ssl-selfsigned.log }
if ($LASTEXITCODE) { Throw }
sleep 5
if (netstat -ano | Select-String LISTENING | Select-string 127.0.0.1:8001) {powershell write-host -foregroundcolor Green "server-ssl-selfsigned up"} else {powershell write-host -foregroundcolor Red "server-ssl-selfsigned is down" ; exit 1}

Start-Process powershell -WindowStyle Hidden { python ssl/server.py 8002 ssl/server/cert.pem false 2>&1 > build\server-ssl-signedbyca.log }
if ($LASTEXITCODE) { Throw }
sleep 5
if (netstat -ano | Select-String LISTENING | Select-string 127.0.0.1:8002) {powershell write-host -foregroundcolor Green "server-ssl-signedbyca up"} else {powershell write-host -foregroundcolor Red "server-ssl-signedbyca is down" ; exit 1}

Start-Process powershell -WindowStyle Hidden { python ssl/server.py 8003 ssl/server/cert.pem true 2>&1 > build\server-ssl-client-authent.log }
if ($LASTEXITCODE) { Throw }
sleep 5
if (netstat -ano | Select-String LISTENING | Select-string 127.0.0.1:8003) {powershell write-host -foregroundcolor Green "server-ssl-client-authent up"} else {powershell write-host -foregroundcolor Red "server-ssl-client-authent is down" ; exit 1}

cd $actual_dir

