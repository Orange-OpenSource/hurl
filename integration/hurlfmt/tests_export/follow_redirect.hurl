GET http://localhost:8000/follow-redirect
HTTP 200
`Followed redirect!`

# On 301, 302, 303, redirected request switch to GET.
# Otherwise, method are untouched.
POST http://localhost:8000/follow-redirect
HTTP 200
`Followed redirect!`

POST http://localhost:8000/follow-redirect-308
HTTP 200
`Followed redirect POST!`
