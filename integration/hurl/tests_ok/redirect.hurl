GET http://localhost:8000/redirected
HTTP 200
[Asserts]
url == "http://localhost:8000/redirected"
`Redirected`


# Absolute redirects

GET http://localhost:8000/redirect-absolute
HTTP 302
Location: http://localhost:8000/redirected
[Asserts]
url == "http://localhost:8000/redirect-absolute"


GET http://localhost:8000/redirect-absolute
[Options]
location: true
HTTP 200
[Asserts]
url == "http://localhost:8000/redirected"
`Redirected`


# Relative redirects

GET http://localhost:8000/redirect-relative
HTTP 302
Location: /redirected
[Asserts]
url == "http://localhost:8000/redirect-relative"


GET http://localhost:8000/redirect-relative
[Options]
location: true
HTTP 200
[Asserts]
url == "http://localhost:8000/redirected"
`Redirected`
