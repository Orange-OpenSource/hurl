GET http://localhost:8000/redirected
HTTP/1.0 200
[Asserts]
url == "http://localhost:8000/redirected"
```Redirected```


# Absolute redirects

GET http://localhost:8000/redirect-absolute
HTTP/1.0 302
Location: http://localhost:8000/redirected
[Asserts]
url == "http://localhost:8000/redirect-absolute"

GET http://localhost:8000/redirect-absolute
[Options]
location: true

HTTP/1.0 200
[Asserts]
url == "http://localhost:8000/redirected"
```Redirected```


# Relative redirects

GET http://localhost:8000/redirect-relative
HTTP/1.0 302
Location: /redirected
[Asserts]
url == "http://localhost:8000/redirect-relative"

GET http://localhost:8000/redirect-relative
[Options]
location: true

HTTP/1.0 200
[Asserts]
url == "http://localhost:8000/redirected"
```Redirected```
