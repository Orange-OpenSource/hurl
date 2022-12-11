#!/bin/bash
set -Eeuo pipefail

color_green=$(echo -ne "\033[1;32m")
color_red=$(echo -ne "\033[1;31m")
color_reset=$(echo -ne "\033[0m")

function check_listen_port(){
    # vars
    label="${1:-}"
    port="${2:-}"
    wait=5

    # usage
    if [ -z "${label}" ] || [ -z "${port}" ] ; then
        echo "${color_red}Usage:${color_reset} check_listen_port {label} {port} {command}"
        return 1
    fi

    sleep "${wait}"
    if nc -zv 127.0.0.1 "${port}" ; then
        echo "${color_green}${label} listenning${color_reset} on ${port}"
        return 0
    else
        echo "${color_red}${label} not listening${color_reset} on ${port}"
        return 1
    fi
}

echo "----- install servers prerequisites -----"
pip3 install --requirement bin/requirements-frozen.txt

echo "----- start servers -----"
cd integration

echo -e "\n------------------ Starting server.py"
(python3 server.py >server.log 2>&1 || true) &
check_listen_port "server.py" 8000

echo -e "\n------------------ Starting ssl/server.py (Self-signed certificate)"
(python3 ssl/server.py 8001 ssl/server/cert.selfsigned.pem false > server-ssl-selfsigned.log 2>&1 || true) &
check_listen_port "ssl/server.py" 8001

echo -e "\n------------------ Starting ssl/server.py (Signed by CA)"
(python3 ssl/server.py 8002 ssl/server/cert.pem false > server-ssl-signedbyca.log 2>&1 || true) &
check_listen_port "ssl/server.py" 8002

echo -e "\n------------------ Starting mitmdump"
(mitmdump --listen-host 127.0.0.1 --listen-port 8888 --modify-header "/From-Proxy/Hello" >mitmproxy.log 2>&1 ||true) &
check_listen_port "mitmdump" 8888

