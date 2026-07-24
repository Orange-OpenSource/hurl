# <https://google.com> has sporadic connection resets (see <https://curl.se/mail/lib-2024-07/0035.html>)
# so we use Cloudflare HTTP/3 test page now.
HEAD https://cloudflare-quic.com
HTTP/3 *

