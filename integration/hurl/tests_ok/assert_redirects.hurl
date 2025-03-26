GET http://localhost:8000/redirect-3
[Options]
location: true
HTTP 200
[Asserts]
redirects count == 3
