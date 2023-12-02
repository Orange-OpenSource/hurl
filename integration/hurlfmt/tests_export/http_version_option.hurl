GET http://localhost:8000/http_version/10
[Options]
http1.0: true
HTTP 200

# RFC 2616 requires that HTTP servers always begin their responses with
# the highest HTTP version that they claim to support. So the HTTP version can
# be different from what the client used in their request.
# See <https://serverfault.com/questions/1059870/why-does-apache-send-http-1-1-when-client-requests-http-1-0-and-many-other-very>
# We validate on the Flask endpoint `http_version.py` that the version asked by the client is the right one.
GET http://localhost:8000/http_version/10
[Options]
http1.0: true
HTTP/1.1 200

# Default HTTP version for Flask is HTTP/1.1
GET http://localhost:8000/http_version/11
HTTP 200

GET http://localhost:8000/http_version/11
[Options]
http1.1: true
HTTP 200

# The last option win
GET http://localhost:8000/http_version/11
[Options]
http1.0: true
http1.1: true
HTTP 200

GET http://localhost:8000/http_version/11
[Options]
http1.0: false
HTTP 200

# Live test on google.fr
HEAD https://google.com
[Options]
http1.0: true
HTTP/1.0 *

HEAD https://google.com
[Options]
http1.1: true
HTTP/1.1 *
