# By default, redirect are not followed
GET http://localhost:8000/follow-redirect
Accept: text/plain
HTTP 302
[Asserts]
header "Location" == "http://localhost:8000/following-redirect"


GET http://localhost:8000/follow-redirect
Accept: text/plain
[Options]
location: false
HTTP 302
[Asserts]
header "Location" == "http://localhost:8000/following-redirect"


GET http://localhost:8000/follow-redirect
Accept: text/plain
[Options]
location: true
HTTP 200
[Asserts]
header "Location" not exists
`Followed redirect!`


# On 301, 302, 303, redirected request switch to GET.
# Otherwise, method are untouched.
POST http://localhost:8000/follow-redirect
Accept: text/plain
[Options]
location: true
HTTP 200
[Asserts]
header "Location" not exists
`Followed redirect!`


POST http://localhost:8000/follow-redirect-308
[Options]
location: true
HTTP 200
[Asserts]
header "Location" not exists
`Followed redirect POST!`

# Do not forward authorization header by default toa different host
GET http://localhost:8000/follow-redirect-basic-auth
Authorization: Basic Ym9iQGVtYWlsLmNvbTpzZWNyZXQ=
[Options]
location: true
HTTP 200
[Asserts]
header "Location" not exists
`Followed redirect Basic Auth!`
