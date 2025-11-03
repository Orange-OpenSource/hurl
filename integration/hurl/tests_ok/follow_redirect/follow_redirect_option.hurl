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
[Query]
# We add a query param to test that query params strings are not forwarded
foo: bar
HTTP 200
[Asserts]
header "Location" not exists
redirects count == 2
redirects nth 0 location == "http://localhost:8000/following-redirect"
redirects nth 1 location == "http://localhost:8000/followed-redirect"
url == "http://localhost:8000/followed-redirect"
`Followed redirect!`


# On 301, 302, 303, redirected request switch to GET.
# Otherwise, method are untouched.
POST http://localhost:8000/follow-redirect-from-post
Accept: text/plain
[Options]
location: true
`Hello world!`
HTTP 200
[Asserts]
header "Location" not exists
redirects count == 1
redirects nth 0 location == "http://localhost:8000/followed-redirect-from-post"
url == "http://localhost:8000/followed-redirect-from-post"
`Followed redirect!`


POST http://localhost:8000/follow-redirect-308
[Options]
location: true
[Form]
user: bob
pass: 123
HTTP 200
[Asserts]
header "Location" not exists
redirects count == 1
redirects nth 0 location == "http://localhost:8000/followed-redirect-308"
url == "http://localhost:8000/followed-redirect-308"
`Followed redirect 308!`


GET http://localhost:8000/follow-redirect/relative/foo
Accept: text/plain
[Options]
location: true
HTTP 200
[Asserts]
redirects nth 0 location == "http://localhost:8000/follow-redirect/bar"
redirects count == 1
url == "http://localhost:8000/follow-redirect/bar"
`Followed relative redirect!`


# Do not forward `Authorization` header to a different host
GET http://localhost:8000/follow-redirect-basic-auth?change_host=true
Authorization: Basic Ym9iQGVtYWlsLmNvbTpzZWNyZXQ=
[Options]
location: true
[Cookies]
fruit: lemon
HTTP 200
[Asserts]
redirects count == 1
redirects nth 0 location == "http://127.0.0.1:8000/followed-redirect-basic-auth"
url == "http://127.0.0.1:8000/followed-redirect-basic-auth"
header "Location" not exists
`Followed redirect without Authorization header!`


# Same test as previous, but the header name case (`Authorization`) should not matter, this header should be filtered
# no matter what's the case is.
GET http://localhost:8000/follow-redirect-basic-auth?change_host=true
# Authorization header in lowercase.
authorization: Basic Ym9iQGVtYWlsLmNvbTpzZWNyZXQ=
[Options]
location: true
[Cookies]
fruit: lemon
HTTP 200
[Asserts]
redirects count == 1
redirects nth 0 location == "http://127.0.0.1:8000/followed-redirect-basic-auth"
url == "http://127.0.0.1:8000/followed-redirect-basic-auth"
header "Location" not exists
`Followed redirect without Authorization header!`


# Same has previous but the host doesn't change during redirection.
# Back checks will insure that `Authorization` is forwarded.
GET http://localhost:8000/follow-redirect-basic-auth?change_host=false
Authorization: Basic Ym9iQGVtYWlsLmNvbTpzZWNyZXQ=
[Options]
location: true
[Cookies]
fruit: lemon
HTTP 200
[Asserts]
redirects count == 1
redirects nth 0 location == "http://localhost:8000/followed-redirect-basic-auth"
url == "http://localhost:8000/followed-redirect-basic-auth"
header "Location" not exists
`Followed redirect with Authorization header!`


# Another kinds of user authentication with `--user` in `[Options]` section:
GET http://localhost:8000/follow-redirect-basic-auth?change_host=true
[Options]
location: true
user: bob@email.com:secret
[Cookies]
fruit: lemon
HTTP 200
[Asserts]
redirects count == 1
redirects nth 0 location == "http://127.0.0.1:8000/followed-redirect-basic-auth"
url == "http://127.0.0.1:8000/followed-redirect-basic-auth"
header "Location" not exists
`Followed redirect without Authorization header!`


GET http://localhost:8000/follow-redirect-basic-auth?change_host=false
[Options]
location: true
user: bob@email.com:secret
[Cookies]
fruit: lemon
HTTP 200
[Asserts]
header "Location" not exists
redirects count == 1
redirects nth 0 location == "http://localhost:8000/followed-redirect-basic-auth"
url == "http://localhost:8000/followed-redirect-basic-auth"
`Followed redirect with Authorization header!`


# Another kinds of user authentication with `[BasicAuth]` section:
GET http://localhost:8000/follow-redirect-basic-auth?change_host=true
[Options]
location: true
[BasicAuth]
bob@email.com: secret
[Cookies]
fruit: lemon
HTTP 200
[Asserts]
redirects count == 1
redirects nth 0 location == "http://127.0.0.1:8000/followed-redirect-basic-auth"
url == "http://127.0.0.1:8000/followed-redirect-basic-auth"
header "Location" not exists
`Followed redirect without Authorization header!`


GET http://localhost:8000/follow-redirect-basic-auth?change_host=false
[Options]
location: true
[BasicAuth]
bob@email.com: secret
[Cookies]
fruit: lemon
HTTP 200
[Asserts]
redirects count == 1
redirects nth 0 location == "http://localhost:8000/followed-redirect-basic-auth"
url == "http://localhost:8000/followed-redirect-basic-auth"
header "Location" not exists
`Followed redirect with Authorization header!`


# Forward authorization header to a different host explicitly
GET http://localhost:8000/follow-redirect-basic-auth-trusted
Authorization: Basic Ym9iQGVtYWlsLmNvbTpzZWNyZXQ=
[Options]
location-trusted: true
[Cookies]
fruit: lemon
HTTP 200
[Asserts]
redirects count == 1
redirects nth 0 location == "http://127.0.0.1:8000/followed-redirect-basic-auth-trusted"
url == "http://127.0.0.1:8000/followed-redirect-basic-auth-trusted"
header "Location" not exists
`Followed redirect Basic Auth!`
