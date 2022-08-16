# By default, redirect are not followed
GET http://localhost:8000/follow-redirect

HTTP/1.0 302
[Asserts]
header "Location" == "http://localhost:8000/following-redirect"


GET http://localhost:8000/follow-redirect
[Options]
location: false

HTTP/1.0 302
[Asserts]
header "Location" == "http://localhost:8000/following-redirect"


GET http://localhost:8000/follow-redirect
[Options]
location: true

HTTP/1.0 200
[Asserts]
header "Location" not exists
```Followed redirect!```

