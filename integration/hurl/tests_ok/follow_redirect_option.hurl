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
Accept: text/plain
[Options]
location: true
HTTP 200
[Asserts]
header "Location" not exists
`Followed redirect POST!`


GET http://localhost:8000/follow-redirect/relative/foo
Accept: text/plain
[Options]
location: true
HTTP 200
[Asserts]
url == "http://localhost:8000/follow-redirect/bar"
`Followed relative redirect!`


# Do not forward authorization header by default to a different host
GET http://localhost:8000/follow-redirect-basic-auth
Authorization: Basic Ym9iQGVtYWlsLmNvbTpzZWNyZXQ=
[Options]
location: true
HTTP 200
[Asserts]
header "Location" not exists
`Followed redirect Basic Auth!`


# Forward authorization header to a different host explicitly
GET http://localhost:8000/follow-redirect-basic-auth-trusted
Authorization: Basic Ym9iQGVtYWlsLmNvbTpzZWNyZXQ=
[Options]
location-trusted: true
HTTP 200
[Asserts]
header "Location" not exists
`Followed redirect Basic Auth!`
