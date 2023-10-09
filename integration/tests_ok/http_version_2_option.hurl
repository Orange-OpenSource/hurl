# Live tests on google.com
HEAD https://google.com
HTTP/2 *


HEAD https://google.com
[Options]
http2: true
HTTP/2 *


HEAD https://google.com
[Options]
http2: false
HTTP/1.1 *


HEAD https://google.com
HTTP/2 *


HEAD https://google.com
HTTP/2 *
