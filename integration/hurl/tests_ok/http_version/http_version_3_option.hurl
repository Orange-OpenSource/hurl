# Live tests on google.com
HEAD https://google.com
HTTP/2 *


HEAD https://google.com
[Options]
http3: true
# We put retry here because we have sporadic connection resets (see <https://curl.se/mail/lib-2024-07/0035.html>)
# 13/08/2025: same error on Debian Trixie
# 17/02/2025: error on Arch Linux
# The GitHub runner are supporting HTTP/3 but we get this error:
# ```shell
#  expected: 0  actual:3
#  error: HTTP connection
#    --> tests_ok/http_version_3_option.hurl:6:6
#     |
#   6 | HEAD https://google.com
#     |      ^^^^^^^^^^^^^^^^^^ (95) HTTP/3 stream 0 reset by server
#     |
# ```
retry: 10
retry-interval: 1s
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
