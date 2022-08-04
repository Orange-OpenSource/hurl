#!/bin/sh
set -e
echo "----- install servers prerequisites -----"
pip3 install --requirement integration/requirements-frozen.txt
echo "----- start servers -----"
cd integration
python3 server.py >server.log 2>&1 &
python3 ssl/server.py >server-ssl.log 2>&1 &
mitmdump -p 8888 --modify-header "/From-Proxy/Hello" >mitmdump.log 2>&1 &


