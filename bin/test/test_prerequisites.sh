#!/bin/bash
set -e
echo "----- install servers prerequisites -----"
pip3 install --requirement bin/requirements-frozen.txt
echo "----- start servers -----"
cd integration
python3 server.py >server.log 2>&1 &
python3 ssl/server.py >server-ssl.log 2>&1 &
mitmdump --listen-host 127.0.0.1 --listen-port 8888 --modify-header "/From-Proxy/Hello" >mitmdump.log 2>&1 &


