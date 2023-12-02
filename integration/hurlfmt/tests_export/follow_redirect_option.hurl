# By default, redirect are not followed
GET http://localhost:8000/follow-redirect

HTTP 302
[Asserts]
header "Location" == "http://localhost:8000/following-redirect"


GET http://localhost:8000/follow-redirect
[Options]
location: false

HTTP 302
[Asserts]
header "Location" == "http://localhost:8000/following-redirect"


GET http://localhost:8000/follow-redirect
[Options]
location: true

HTTP 200
[Asserts]
header "Location" not exists
`Followed redirect!`


# On 301, 302, 303, redirected request switch to GET.
# Otherwise, method are untouched.
POST http://localhost:8000/follow-redirect
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
