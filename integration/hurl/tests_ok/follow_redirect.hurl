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

# Check relative redirection
GET http://localhost:8000/follow-redirect/relative/foo
Accept: text/plain
HTTP 200
[Asserts]
url == "http://localhost:8000/follow-redirect/bar"
`Followed relative redirect!`


# Do not forward `Authorization` header to a different host
GET http://localhost:8000/follow-redirect-basic-auth?change_host=true
Authorization: Basic Ym9iQGVtYWlsLmNvbTpzZWNyZXQ=
HTTP 200
[Asserts]
header "Location" not exists
`Followed redirect without Authorization header!`


# Same has previous but the host doesn't change during redirection.
# Back checks will insure that `Authorization` is forwarded.
GET http://localhost:8000/follow-redirect-basic-auth?change_host=false
Authorization: Basic Ym9iQGVtYWlsLmNvbTpzZWNyZXQ=
HTTP 200
[Asserts]
header "Location" not exists
`Followed redirect with Authorization header!`


# Another kinds of user authentication with `--user` in `[Options]` section:
GET http://localhost:8000/follow-redirect-basic-auth?change_host=true
[Options]
user: bob@email.com:secret
HTTP 200
[Asserts]
header "Location" not exists
`Followed redirect without Authorization header!`


GET http://localhost:8000/follow-redirect-basic-auth?change_host=false
[Options]
user: bob@email.com:secret
HTTP 200
[Asserts]
header "Location" not exists
`Followed redirect with Authorization header!`


# Another kinds of user authentication with `[BasicAuth]` section:
GET http://localhost:8000/follow-redirect-basic-auth?change_host=true
[BasicAuth]
bob@email.com: secret
HTTP 200
[Asserts]
header "Location" not exists
`Followed redirect without Authorization header!`


GET http://localhost:8000/follow-redirect-basic-auth?change_host=false
[BasicAuth]
bob@email.com: secret
HTTP 200
[Asserts]
header "Location" not exists
`Followed redirect with Authorization header!`


POST http://localhost:8000/follow-redirect-308
Accept: text/plain
HTTP 200
[Asserts]
url == "http://localhost:8000/followed-redirect-post"
`Followed redirect POST!`
