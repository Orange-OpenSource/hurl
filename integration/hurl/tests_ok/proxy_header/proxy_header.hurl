# Hurl will sent the proxy header Foo:Bar

# If the request goes through proxy
# This header will be sent by the server
GET http://127.0.0.1:8000/proxy_header
[Options]
proxy: localhost:3128
HTTP 200
From-Proxy: Hello

# Without a proxy,
# the proxy header will not be sent to the server
GET http://127.0.0.1:8000/proxy_header_direct
[Options]
HTTP 200
