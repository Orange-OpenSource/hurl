from ipaddress import IPv4Address, IPv6Address, ip_address

from app import app
from flask import Response, request


@app.route("/ip")
def ip():
    request_ip = request.remote_addr
    return Response(request_ip)


@app.route("/check-ipv4")
def check_ipv4():
    request_ip = request.remote_addr
    if type(ip_address(request_ip)) is IPv4Address:
        status = 200
    else:
        status = 400
    return Response(request_ip, status=status)


@app.route("/check-ipv6")
def check_ipv6():
    request_ip = request.remote_addr
    if type(ip_address(request_ip)) is IPv6Address:
        status = 200
    else:
        status = 400
    return Response(request_ip, status=status)
