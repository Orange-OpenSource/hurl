#!/bin/bash
set -Eeuo pipefail

color_green=$(echo -ne "\033[1;32m")
color_red=$(echo -ne "\033[1;31m")
color_reset=$(echo -ne "\033[0m")

function check_listen_port(){
    # vars
    label="${1:-}"
    port="${2:-}"

    # usage
    if [ -z "${label}" ] || [ -z "${port}" ] ; then
        echo "${color_red}Usage:${color_reset} check_listen_port {label} {port} {command}"
        return 1
    fi

    for count in $(seq 30) ; do
        if nc -zv 127.0.0.1 "${port}" ; then
            exit_message="${color_green}$(date) - ${label} listening${color_reset} on ${port}"
            exit_code=0
            break
        else
            echo "$(date) - ${count} try - ${label} not listening${color_reset} on ${port} yet"
            exit_message="${color_red}$(date) - ${label} not listening${color_reset} on ${port}"
            exit_code=1
        fi
        sleep 1
    done
    echo "${exit_message}"
    return "${exit_code}"
}

function cat_and_exit_err() {
    file="$1"
    cat "$file"
    return 1
}

echo "----- install servers prerequisites -----"
python3 -m pip install --requirement bin/requirements-frozen.txt

echo "----- start servers -----"
cd integration
mkdir -p build

echo -e "\n------------------ Starting server.py"
python3 server.py > build/server.log 2>&1 &
check_listen_port "server.py" 8000 || cat_and_exit_err build/server.log

echo -e "\n------------------ Starting ssl/server.py (Self-signed certificate)"
python3 ssl/server.py 8001 ssl/server/cert.selfsigned.pem false > build/server-ssl-selfsigned.log 2>&1 &
check_listen_port "ssl/server.py" 8001 || cat_and_exit_err build/server-ssl-selfsigned.log

echo -e "\n------------------ Starting ssl/server.py (Signed by CA)"
python3 ssl/server.py 8002 ssl/server/cert.pem false > build/server-ssl-signedbyca.log 2>&1 &
check_listen_port "ssl/server.py" 8002 || cat_and_exit_err build/server-ssl-signedbyca.log

echo -e "\n------------------ Starting ssl/server.py (Self-signed certificate + Client certificate authentication)"
nohup python3 ssl/server.py 8003 ssl/server/cert.selfsigned.pem true > build/server-ssl-client-authent.log 2>&1 &
check_listen_port "ssl/server.py" 8003 || cat_and_exit_err build/server-ssl-client-authent.log

echo -e "\n------------------ Starting squid (proxy)"
if [ -f /var/run/squid.pid ] ; then
  sudo squid -k shutdown || true
  sudo kill -9 "$(cat /var/run/squid.pid || true)" || true
  sudo rm -fr /var/run/squid.pid || true
fi
squid_conf="cache deny all\ncache_log /dev/null\naccess_log /dev/null\nhttp_access allow all\nhttp_port 127.0.0.1:3128\nrequest_header_add From-Proxy Hello\nreply_header_add From-Proxy Hello"
(echo -e "${squid_conf}" | sudo squid -d 2 -N -f /dev/stdin | sudo tee build/proxy.log 2>&1) &
check_listen_port "squid" 3128 || cat_and_exit_err build/proxy.log
