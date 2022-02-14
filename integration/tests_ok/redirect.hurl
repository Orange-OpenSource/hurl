GET http://localhost:8000/redirect
HTTP/1.0 302
Location: http://localhost:8000/redirected

GET http://localhost:8000/redirected
HTTP/1.0 200






