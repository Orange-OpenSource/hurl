echo "----- install tests prerequisites -----"

$actual_dir=(Get-Location).Path

# install python libs
pip3 install --requirement bin\requirements-frozen.txt

# start mock servers
cd $PSScriptRoot\..\..\integration
Start-Job -Name mitmdump -ScriptBlock { mitmdump --listen-port 8888 --modify-header "/From-Proxy/Hello" }
Start-Job -Name server -ScriptBlock { python server.py > server.log }
Start-Job -Name server -ScriptBlock { python ssl/server.py > server-ssl.log }
Get-Job -Name server
Get-Job -Name mitmdump
Start-Sleep 5

cd $actual_dir
