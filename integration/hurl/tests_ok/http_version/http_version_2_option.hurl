# Live tests on google.com
HEAD https://google.com
HTTP/2 *


# Same request using explicit version query
HEAD https://google.com
HTTP *
[Asserts]
version == "2"


# Explicit and implicit version combined:
HEAD https://google.com
HTTP/2 *
[Asserts]
version == "2"


# --http2 has no effect on an HTTP/2 server
HEAD https://google.com
[Options]
http2: true
HTTP/2 *


HEAD https://google.com
[Options]
http2: true
HTTP *
[Asserts]
version == "2"


# When http2: false, we downgrade to the lower HTTP version
HEAD https://google.com
[Options]
http2: false
HTTP/1.1 *


HEAD https://google.com
[Options]
http2: false
HTTP *
[Asserts]
version == "1.1"


# Default requests are OK

HEAD https://google.com
HTTP/2 *


HEAD https://google.com
HTTP *
[Asserts]
version == "2"


HEAD https://google.com
HTTP/2 *


HEAD https://google.com
HTTP *
[Asserts]
version == "2"
