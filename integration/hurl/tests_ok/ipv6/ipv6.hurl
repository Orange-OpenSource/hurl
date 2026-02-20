GET http://[::1]:8004/hello/ipv6
HTTP 200
[Asserts]
ip == "::1"
ip isIpv6
