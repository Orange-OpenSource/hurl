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


# Do not forward `Authorization` header to a different host
GET http://localhost:8000/follow-redirect-basic-auth?change_host=true
Authorization: Basic Ym9iQGVtYWlsLmNvbTpzZWNyZXQ=
[Options]
location: true
HTTP 200
[Asserts]
header "Location" not exists
`Followed redirect without Authorization header!`


# Same has previous but the host doesn't change during redirection.
# Back checks will insure that `Authorization` is forwarded.
GET http://localhost:8000/follow-redirect-basic-auth?change_host=false
Authorization: Basic Ym9iQGVtYWlsLmNvbTpzZWNyZXQ=
[Options]
location: true
HTTP 200
[Asserts]
header "Location" not exists
`Followed redirect with Authorization header!`


# Another kinds of user authentication with `--user` in `[Options]` section:
GET http://localhost:8000/follow-redirect-basic-auth?change_host=true
[Options]
location: true
user: bob@email.com:secret
HTTP 200
[Asserts]
header "Location" not exists
`Followed redirect without Authorization header!`


GET http://localhost:8000/follow-redirect-basic-auth?change_host=false
[Options]
location: true
user: bob@email.com:secret
HTTP 200
[Asserts]
header "Location" not exists
`Followed redirect with Authorization header!`


# Another kinds of user authentication with `[BasicAuth]` section:
GET http://localhost:8000/follow-redirect-basic-auth?change_host=true
[Options]
location: true
[BasicAuth]
bob@email.com: secret
HTTP 200
[Asserts]
header "Location" not exists
`Followed redirect without Authorization header!`


GET http://localhost:8000/follow-redirect-basic-auth?change_host=false
[Options]
location: true
[BasicAuth]
bob@email.com: secret
HTTP 200
[Asserts]
header "Location" not exists
`Followed redirect with Authorization header!`


# Forward authorization header to a different host explicitly
GET http://localhost:8000/follow-redirect-basic-auth-trusted
Authorization: Basic Ym9iQGVtYWlsLmNvbTpzZWNyZXQ=
[Options]
location-trusted: true
HTTP 200
[Asserts]
header "Location" not exists
`Followed redirect Basic Auth!`
