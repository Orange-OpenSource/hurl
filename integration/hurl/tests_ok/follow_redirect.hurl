# Hurl implements curl's behaviour on redirect:
#
# From <https://curl.se/docs/manpage.html#-L>:
# > When curl follows a redirect and if the request is a POST, it sends the following request
# > with a GET if the HTTP response was 301, 302, or 303. If the response code was any other 3xx
# > code, curl resends the following request using the same unmodified method.
#
# In the following test, we send an `Accept` header to check that any header (except `Authorization`
# are redirected to the final URL.

GET http://localhost:8000/follow-redirect
Accept: text/plain
HTTP 200
[Asserts]
url == "http://localhost:8000/followed-redirect"
`Followed redirect!`


# On 301, 302, 303, redirected request switch to GET.
# Otherwise, method are untouched.
POST http://localhost:8000/follow-redirect
Accept: text/plain
HTTP 200
[Asserts]
url == "http://localhost:8000/followed-redirect"
`Followed redirect!`


GET http://localhost:8000/follow-redirect/relative/foo
Accept: text/plain
HTTP 200
[Asserts]
url == "http://localhost:8000/follow-redirect/bar"
`Followed relative redirect!`


# Do not forward authorization header by default to a different host
GET http://localhost:8000/follow-redirect-basic-auth
Authorization: Basic Ym9iQGVtYWlsLmNvbTpzZWNyZXQ=
HTTP 200
[Asserts]
header "Location" not exists
`Followed redirect Basic Auth!`


# Another kinds of user authentication:
GET http://localhost:8000/follow-redirect-basic-auth
[Options]
user: bob@email.com:secret
HTTP 200
[Asserts]
header "Location" not exists
`Followed redirect Basic Auth!`


GET http://localhost:8000/follow-redirect-basic-auth
[BasicAuth]
bob@email.com: secret
HTTP 200
[Asserts]
header "Location" not exists
`Followed redirect Basic Auth!`




POST http://localhost:8000/follow-redirect-308
Accept: text/plain
HTTP 200
[Asserts]
url == "http://localhost:8000/followed-redirect-post"
`Followed redirect POST!`
