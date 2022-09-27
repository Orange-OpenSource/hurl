powershell write-host -foregroundcolor Cyan "----- install tests prerequisites -----"

$actual_dir=(Get-Location).Path
$project_root_path=(Resolve-Path -LiteralPath $PSScriptRoot\..\..).path

# install python libs
pip3 install --requirement $project_root_path\bin\requirements-frozen.txt

# start mock servers
cd $project_root_path\integration
Start-Process powershell -WindowStyle Hidden { mitmdump --listen-port 8888 --modify-header "/From-Proxy/Hello" 2>&1 > mitmdump.log }
sleep 5
if (netstat -ano | Select-String LISTENING | Select-string 0.0.0.0:8888) {powershell write-host -foregroundcolor Green "mitmdump is up"} else {powershell write-host -foregroundcolor Red "mitmdump is down" ; exit 1}
Start-Process powershell -WindowStyle Hidden { python server.py 2>&1 > server.log }
sleep 5
if (netstat -ano | Select-String LISTENING | Select-string 127.0.0.1:8000) {powershell write-host -foregroundcolor Green "server is up"} else {powershell write-host -foregroundcolor Red "server is down" ; exit 1}
Start-Process powershell -WindowStyle Hidden { python ssl/server.py 2>&1 > server-ssl.log }
sleep 5
if (netstat -ano | Select-String LISTENING | Select-string 127.0.0.1:8001) {powershell write-host -foregroundcolor Green "server-ssl up"} else {powershell write-host -foregroundcolor Red "server-ssl is down" ; exit 1}

cd $actual_dir

