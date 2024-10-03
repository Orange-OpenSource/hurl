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
    echo -e "\n${exit_message}"
    return "${exit_code}"
}

function check_unix_socket(){
    # vars
    label="${1:-}"
    socket_file="${2:-}"
    path="${3:-}"

    # usage
    if [ -z "${label}" ] || [ -z "${socket_file}" ] ; then
        echo "${color_red}Usage:${color_reset} check_unix_socket {label} {socket_file} {command}"
        return 1
    fi

    for count in $(seq 30) ; do
        if echo -e "${path}" | nc -U "${socket_file}" ; then
            exit_message="${color_green}$(date) - ${label} listening${color_reset} on ${socket_file}"
            exit_code=0
            break
        else
            echo "$(date) - ${count} try - ${label} not listening${color_reset} on ${socket_file} yet"
            exit_message="${color_red}$(date) - ${label} not listening${color_reset} on ${socket_file}"
            exit_code=1
        fi
        sleep 1
    done
    echo -e "\n${exit_message}"
    return "${exit_code}"
}

function cat_and_exit_err() {
    file="$1"
    cat "$file"
    return 1
}

echo -e "\n----- install servers prerequisites -----"
python3 -m pip install --requirement bin/requirements-frozen.txt

echo -e "\n----- start servers -----"
cd integration/hurl
mkdir -p build

echo -e "\n------------------ Starting server.py"
python3 server.py > build/server.log 2>&1 &
check_listen_port "server.py" 8000 || cat_and_exit_err build/server.log

echo -e "\n------------------ Starting tests_ssl/ssl_server.py (Self-signed certificate)"
python3 tests_ssl/ssl_server.py 8001 tests_ssl/certs/server/cert.selfsigned.pem false > build/server-ssl-selfsigned.log 2>&1 &
check_listen_port "tests_ssl/ssl_server.py" 8001 || cat_and_exit_err build/server-ssl-selfsigned.log

echo -e "\n------------------ Starting tests_ssl/ssl_server.py (Signed by CA)"
python3 tests_ssl/ssl_server.py 8002 tests_ssl/certs/server/cert.pem false > build/server-ssl-signedbyca.log 2>&1 &
check_listen_port "tests_ssl/ssl_server.py" 8002 || cat_and_exit_err build/server-ssl-signedbyca.log

echo -e "\n------------------ Starting ssl/ssl_server.py (Self-signed certificate + Client certificate authentication)"
python3 tests_ssl/ssl_server.py 8003 tests_ssl/certs/server/cert.selfsigned.pem true > build/server-ssl-client-authent.log 2>&1 &
check_listen_port "tests_ssl/ssl_server.py" 8003 || cat_and_exit_err build/server-ssl-client-authent.log

echo -e "\n------------------ Starting tests_unix_socket/unix_socket_server.py"
python3 tests_unix_socket/unix_socket_server.py > build/server-unix-socket.log 2>&1 &
check_unix_socket "tests_unix_socket/unix_socket_server.py" build/unix_socket.sock "GET /hello HTTP/1.0\r\n"

echo -e "\n------------------ Starting squid (proxy)"
if [ -f /var/run/squid.pid ] ; then
  sudo squid -k shutdown || true
  sudo kill -9 "$(cat /var/run/squid.pid || true)" || true
  sudo rm -fr /var/run/squid.pid || true
fi
squid_conf="cache deny all\ncache_log /dev/null\naccess_log /dev/null\nhttp_access allow all\nhttp_port 127.0.0.1:3128\nrequest_header_add From-Proxy Hello\nreply_header_add From-Proxy Hello"
(echo -e "${squid_conf}" | sudo squid -d 2 -N -f /dev/stdin | sudo tee build/proxy.log 2>&1) &
check_listen_port "squid" 3128 || cat_and_exit_err build/proxy.log

