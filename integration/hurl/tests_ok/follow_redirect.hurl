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
HTTP 200
[Asserts]
url == "http://localhost:8000/follow-redirect/bar"
`Followed relative redirect!`


POST http://localhost:8000/follow-redirect-308
HTTP 200
[Asserts]
url == "http://localhost:8000/followed-redirect-post"
`Followed redirect POST!`
