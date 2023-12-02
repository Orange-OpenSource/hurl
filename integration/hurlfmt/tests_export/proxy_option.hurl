# Go through proxy
# The proxy adds header "From-Proxy:Hello" for both request and response
GET http://127.0.0.1:8000/proxy
[Options]
proxy: localhost:3128
HTTP 200
From-Proxy: Hello

# Using ipv4
GET http://127.0.0.1:8000/proxy
[Options]
proxy: 127.0.0.1:3128
HTTP 200
From-Proxy: Hello
