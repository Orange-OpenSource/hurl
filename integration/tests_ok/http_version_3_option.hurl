# Live tests on google.com
HEAD https://google.com
HTTP/2 *


HEAD https://google.com
[Options]
http3: true
HTTP/3 *


HEAD https://google.com
[Options]
http3: false
HTTP/2 *


# Downgrade to HTTP 2 is working
HEAD https://google.com
[Options]
http2: true
HTTP/2 *


# Downgrade to HTTP 1.1 is working
HEAD https://google.com
[Options]
http1.1: true
HTTP/1.1 *
