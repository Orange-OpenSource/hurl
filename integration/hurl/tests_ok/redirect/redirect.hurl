GET http://localhost:8000/redirected
HTTP 200
[Asserts]
url == "http://localhost:8000/redirected"
redirects count == 0
`Redirected`


# Absolute redirects

GET http://localhost:8000/redirect-absolute
HTTP 302
Location: http://localhost:8000/redirected
[Asserts]
url == "http://localhost:8000/redirect-absolute"
# `redirects` query extracts data only if Hurl has run redirection (using --location or --location-trusted)
# If this case we have a 302, but we've nots asked Hurl to follow redirection so `redirects` query is empty.
redirects count == 0


GET http://localhost:8000/redirect-absolute
[Options]
location: true
HTTP 200
[Asserts]
url == "http://localhost:8000/redirected"
redirects count == 1
redirects nth 0 location == "http://localhost:8000/redirected"
`Redirected`


# Redirection can redirect body from requests to requests (provided the method doesn't
# change) but query strings params are NEVER forwarded
GET http://localhost:8000/redirect-absolute
[Options]
location: true
[Query]
foo: bar
HTTP 200
[Asserts]
url == "http://localhost:8000/redirected"
redirects count == 1
redirects nth 0 location == "http://localhost:8000/redirected"
`Redirected`


# Relative redirects

GET http://localhost:8000/redirect-relative
HTTP 302
Location: /redirected
[Asserts]
url == "http://localhost:8000/redirect-relative"
redirects count == 0


GET http://localhost:8000/redirect-relative
[Options]
location: true
HTTP 200
[Asserts]
url == "http://localhost:8000/redirected"
redirects count == 1
redirects nth 0 location == "http://localhost:8000/redirected"
`Redirected`
