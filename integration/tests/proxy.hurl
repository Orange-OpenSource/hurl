# Go through proxy
# The proxy adds header "From-Proxy:Hello" for both request and response
# mitmproxy -p 8888 --setheader ":~q:From-Proxy:Hello" --setheader ":~s:From-Proxy:Hello"
GET http://localhost:8000/proxy

HTTP/1.0 200
From-Proxy: Hello


