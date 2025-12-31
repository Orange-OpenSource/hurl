# Live tests on google.com
HEAD https://google.com
HTTP/2 *


# <https://google.com> has sporadic connection resets (see <https://curl.se/mail/lib-2024-07/0035.html>)
# so we use Cloudflare HTTP/3 test page now.
HEAD https://cloudflare-quic.com
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
