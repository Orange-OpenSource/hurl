GET http://localhost:8000/hello
HTTP 200
[Captures]
ip_addr: ip
[Asserts]
ip isIpv4
ip not isIpv6
ip == "127.0.0.1"
ip == {{ip_addr}}
variable "ip_addr" matches /^(\d{1,3}\.){3}\d{1,3}$/


# Disabled due to unreliable behavior in the GitHub CI/CD
# GET https://google.com
# [Options]
# ipv6: true
# HTTP *
# [Captures]
# ip_addr: ip
# [Asserts]
# ip isIpv6
# ip not isIpv4
# ip == {{ip_addr}}
# variable "ip_addr" contains ":"


GET https://hurl.dev
HTTP 200
[Captures]
ip_addr: ip
[Asserts]
ip isIpv4
ip not isIpv6
ip == "145.239.78.213"
ip == {{ip_addr}}
variable "ip_addr" == "145.239.78.213"
